mod constants;

use constants::{AES_SBOX as AES_SBOX};
use constants::{INVERSE_AES_SBOX as INVERSE_AES_SBOX};
use constants::{RC as RC};
use std::convert::AsMut;


pub struct AES128 {
  expanded_key: [[u8;4];44],
  pub encrypt: fn(&AES128, &[u8]) -> Vec<u8>,
  pub decrypt: fn(&AES128, &[u8]) -> Vec<u8>,
  encrypt_block: fn(&AES128, &[u8;16]) -> [u8;16],
  decrypt_block: fn(&AES128, &[u8;16]) -> [u8;16],
}


impl AES128 {
  pub fn new_from_str(key: &str) -> AES128 {
      let key_bytes = key.as_bytes();
      if key_bytes.len() != 16 {
          panic!("Key needs to be 16 bytes long");
      } 

      return AES128 {
          expanded_key: key_schedule_AES128(&clone_into_array(key_bytes)),
          encrypt: encrypt_AES128,
          decrypt: decrypt_AES128,
          encrypt_block: encrypt_block_AES128,
          decrypt_block: decrypt_block_AES128,
      }
  }

  pub fn new(key: &[u8; 16]) -> AES128 {
      return AES128 {
          expanded_key: key_schedule_AES128(key),
          encrypt: encrypt_AES128,
          decrypt: decrypt_AES128,
          encrypt_block: encrypt_block_AES128,
          decrypt_block: decrypt_block_AES128,
      }
  }
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
  A: Default + AsMut<[T]>,
  T: Clone,
{
  let mut a = A::default();
  <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
  a
}

fn key_schedule_AES128(key_bytes: &[u8;16]) -> [[u8;4];44] {
  let mut original_key = [[0u8;4];4];
  let mut expanded_key = [[0u8;4];44];
  let N = 4;

  for i in 0..16 {
      original_key[i/4][i%4] = key_bytes[i];
  }

  for i in 0..44 { // 11 rounds, i in 0..4*rounds-1

      if i < N {
          expanded_key[i] = original_key[i];
      } else if  i >= N && i % N == 0 {

          let mut rcon = [0u8;4];
          rcon[0] = RC[i/N];
          expanded_key[i] = xor_words(&xor_words(&expanded_key[i-N], &sub_word(&rot_word(&expanded_key[i-1]))), &rcon);

      } else {
          expanded_key[i] = xor_words(&expanded_key[i-N],&expanded_key[i-1]);
      }
      
  }

  return expanded_key;
}

fn substitute(byte: u8, encryption: bool) -> u8 {
  let upper_nibble : usize;
  let lower_nibble : usize;
  upper_nibble = ((byte>>4) & 0xF).into();
  lower_nibble = (byte & 0xF).into();
  if encryption == true {
      return AES_SBOX[upper_nibble][lower_nibble];
  } else {
      return INVERSE_AES_SBOX[upper_nibble][lower_nibble];
  }
}

fn rot_word(word: &[u8; 4]) -> [u8;4] {
  let mut result = [0u8;4];

  for i in 0..4 {
      result[i] = word[(i+1)%4];
  }

  return result;
}

fn sub_word(word: &[u8; 4]) -> [u8;4] {
  let mut result = [0u8;4];

  for i in 0..4 {
      result[i] = substitute(word[i], true);
  }

  return result;
}

fn xor_words(word1: &[u8; 4], word2: &[u8; 4]) -> [u8;4] {
  let mut result = [0u8;4];

  for i in 0..4 {
      result[i] = word1[i] ^ word2[i];
  }

  return result;
}

fn add_round_key(state:&mut [[u8;4];4], key: &[[u8;4];4]) {
  for i in 0..4 {
      for j in 0..4 {
          state[i][j] = state[i][j] ^ key[j][i];
      }
  }
}

fn sub_bytes(state:&mut [[u8;4];4]) {
  for i in 0..4 {
      for j in 0..4 {
          state[i][j] = substitute(state[i][j], true);
      }
  }
}

fn inv_sub_bytes(state:&mut [[u8;4];4]) {
  for i in 0..4 {
      for j in 0..4 {
          state[i][j] = substitute(state[i][j], false);
      }
  }
}

fn shift_rows(state:&mut [[u8;4];4]) {
  for i in 1..4 {
      let mut tmp = vec![0u8;i];
      for j in 0..i {
          tmp[j] = state[i][j];
      }
      for j in 0..4-i {
          state[i][j] = state[i][j+i];
      }
      for j in 0..i {
          state[i][3-j] = tmp[i-j-1];
      }
  }
}

fn inv_shift_rows(state:&mut [[u8;4];4]) {
  for i in (1..4).rev() {
      let mut tmp = vec![0u8;i];
      for j in 0..i {
          tmp[j] = state[4-i][j];
      }
      for j in 0..4-i {
          state[4-i][j] = state[4-i][j+i];
      }
      for j in 0..i {
          state[4-i][3-j] = tmp[i-j-1];
      }
  }
}

fn galois_multiplication(ap: u8, bp: u8) -> u8 {
  let mut p = 0u8;
  let mut high_bit = 0u8;
  let mut a = ap;
  let mut b = bp;
  for i in 0..8 {
      if b&1 == 1 {
          p ^= a
      }
      high_bit = a & 0x80;
      a = (a<<1) & 0xFF;
      if high_bit == 0x80 {
          a ^= 0x1b;
      }
      b = (b>>1) & 0xFF;
  }
  return p & 0xFF;
}

fn mix_columns(state: &mut [[u8;4];4]) {
  for i in 0..4 {

      let mut temp = [0u8;4];
      for j in 0..4 {
          temp[j] = state[j][i];
      }

      state[0][i] = galois_multiplication(temp[0], 2) ^ galois_multiplication(temp[3], 1) ^ galois_multiplication(temp[2], 1) ^ galois_multiplication(temp[1], 3);
      state[1][i] = galois_multiplication(temp[1], 2) ^ galois_multiplication(temp[0], 1) ^ galois_multiplication(temp[3], 1) ^ galois_multiplication(temp[2], 3);
      state[2][i] = galois_multiplication(temp[2], 2) ^ galois_multiplication(temp[1], 1) ^ galois_multiplication(temp[0], 1) ^ galois_multiplication(temp[3], 3);
      state[3][i] = galois_multiplication(temp[3], 2) ^ galois_multiplication(temp[2], 1) ^ galois_multiplication(temp[1], 1) ^ galois_multiplication(temp[0], 3);

  }
}

fn inv_mix_columns(state: &mut [[u8;4];4]) {
  for i in 0..4 {

      let mut temp = [0u8;4];
      for j in 0..4 {
          temp[j] = state[j][i];
      }

      state[0][i] = galois_multiplication(temp[0], 14) ^ galois_multiplication(temp[3], 9) ^ galois_multiplication(temp[2], 13) ^ galois_multiplication(temp[1], 11);
      state[1][i] = galois_multiplication(temp[1], 14) ^ galois_multiplication(temp[0], 9) ^ galois_multiplication(temp[3], 13) ^ galois_multiplication(temp[2], 11);
      state[2][i] = galois_multiplication(temp[2], 14) ^ galois_multiplication(temp[1], 9) ^ galois_multiplication(temp[0], 13) ^ galois_multiplication(temp[3], 11);
      state[3][i] = galois_multiplication(temp[3], 14) ^ galois_multiplication(temp[2], 9) ^ galois_multiplication(temp[1], 13) ^ galois_multiplication(temp[0], 11);
  }
}

fn encrypt_AES128(aes128: &AES128, bytes: &[u8]) -> Vec<u8> {
  if bytes.len()%16!=0 {
      panic!("Input is not multiple of 16 bytes!");
  }

  let mut result = vec![0u8; bytes.len()];

  for i in 0..bytes.len()/16 {
      let mut block = [0u8;16];
      for j in 0..16 {
          block[j] = bytes[i*16 + j];
      }
      block = encrypt_block_AES128(aes128, &block);
      for j in 0..16 {
          result[i*16 + j] = block[j];
      }
  }

  return result;
}

fn encrypt_block_AES128(aes128: &AES128, bytes: &[u8;16]) -> [u8;16] {
  let mut result = [0u8;16];

  let mut state = [[0u8;4];4];
  for i in 0..16 {
      state[i%4][i/4] = bytes[i];
  }

  add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[0..4]));

  for i in 1..10 {
      sub_bytes(&mut state);
      shift_rows(&mut state);
      mix_columns(&mut state);
      add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[i*4..(i+1)*4]));
  }

  sub_bytes(&mut state);
  shift_rows(&mut state);
  add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[40..44]));

  for i in 0..4 {
      for j in 0..4 {
          result[4*j+i] = state[i][j]
      }
  }

  return result;
}

fn decrypt_AES128(aes128: &AES128, bytes: &[u8]) -> Vec<u8> {
  if bytes.len()%16!=0 {
      panic!("Input is not multiple of 16 bytes!");
  }

  let mut result = vec![0u8; bytes.len()];

  for i in 0..bytes.len()/16 {
      let mut block = [0u8;16];
      for j in 0..16 {
          block[j] = bytes[i*16 + j];
      }
      block = decrypt_block_AES128(aes128, &block);
      for j in 0..16 {
          result[i*16 + j] = block[j];
      }
  }

  return result;
}

fn decrypt_block_AES128(aes128: &AES128, bytes: &[u8;16]) -> [u8;16] {
  let mut result = [0u8;16];

  let mut state = [[0u8;4];4];
  for i in 0..16 {
      state[i%4][i/4] = bytes[i];
  }

  add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[40..44]));
  inv_shift_rows(&mut state);
  inv_sub_bytes(&mut state);

  for i in (1..10).rev() {
      add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[i*4..(i+1)*4]));
      inv_mix_columns(&mut state);
      inv_shift_rows(&mut state);
      inv_sub_bytes(&mut state);
  }

  add_round_key(&mut state, &clone_into_array(&aes128.expanded_key[0..4]));

  for i in 0..4 {
      for j in 0..4 {
          result[4*j+i] = state[i][j]
      }
  }

  return result;
}

pub fn run_test() {
  println!("Testing simple encryption");
  let aes: AES128 = AES128::new_from_str("YellowSubmarine!");

  let mut basic_input = [65u8;16];
  let encryption_result = (aes.encrypt)(&aes, &basic_input);
  assert!(encryption_result == [28, 203, 121, 8, 47, 187, 48, 216, 108, 79, 120, 29, 203, 136, 214, 44]);
  println!("Testing simple encryption - Test OK");

  println!("Testing simple decryption");
  let decryption_result = (aes.decrypt)(&aes, &encryption_result);
  assert!(decryption_result == basic_input);
  println!("Testing simple decryption - Test OK");
}