import { generateMnemonic, mnemonicToSeedSync } from "@scure/bip39";
import { wordlist } from "@scure/bip39/wordlists/english.js";
import { HDKey } from "@scure/bip32";
import { keccak256 } from "ethereum-cryptography/keccak.js";
import { bytesToHex } from "ethereum-cryptography/utils.js";

const mnemonic = process.argv[2] || generateMnemonic(wordlist, 128);
console.log("Mnemonic:", mnemonic);

const seed = mnemonicToSeedSync(mnemonic);
const root = HDKey.fromMasterSeed(seed);

// derivazione standard EVM
const path = "m/44'/60'/0'/0/0";
const child = root.derive(path);

const privateKey = bytesToHex(child.privateKey);
const publicKey = child.publicKey.slice(1);
const address = "0x" + bytesToHex(keccak256(publicKey).slice(-20));

console.log("Private Key:", privateKey);
console.log("Address:", address);
