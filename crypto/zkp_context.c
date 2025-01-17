/**
 * Copyright (c) SatoshiLabs
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
 * OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#include <assert.h>
#include <stdatomic.h>
#include <stdbool.h>

#include "memzero.h"
#include "rand.h"
#include "zkp_context.h"

#include "vendor/secp256k1-zkp/include/secp256k1.h"

static uint8_t context_buffer[SECP256K1_CONTEXT_SIZE];
static secp256k1_context *context;
static volatile atomic_flag locked;

void secp256k1_context_writable_randomize(secp256k1_context *context_writable) {
  uint8_t seed[32] = {0};
  random_buffer(seed, sizeof(seed));
  int returned = secp256k1_context_randomize(context_writable, seed);
  memzero(seed, sizeof(seed));
  assert(returned == 1);
}

bool zkp_context_is_initialized(void) { return context != NULL; }

void zkp_context_init() {
  assert(context == NULL);

  const unsigned int context_flags =
      SECP256K1_CONTEXT_SIGN | SECP256K1_CONTEXT_VERIFY;

  const size_t context_size =
      secp256k1_context_preallocated_size(context_flags);
  assert(context_size != 0);
  assert(context_size <= SECP256K1_CONTEXT_SIZE);

  context =
      secp256k1_context_preallocated_create(context_buffer, context_flags);
  assert(context != NULL);

  secp256k1_context_writable_randomize(context);

  atomic_flag_clear(&locked);
}

void zkp_context_destroy() {
  assert(context != NULL);

  secp256k1_context_preallocated_destroy(context);
  memzero(context_buffer, sizeof(context_buffer));
  atomic_flag_clear(&locked);
}

const secp256k1_context *zkp_context_get_read_only() {
  assert(context != NULL);

  return context;
}

// returns NULL if context cannot be acquired
secp256k1_context *zkp_context_acquire_writable() {
  assert(context != NULL);

  // We don't expect the context to be used by multiple threads
  if (atomic_flag_test_and_set(&locked)) {
    return NULL;
  }

  return context;
}

void zkp_context_release_writable() {
  assert(context != NULL);

  atomic_flag_clear(&locked);
}
