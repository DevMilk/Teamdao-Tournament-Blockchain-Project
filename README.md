npm install
anchor build --provider.cluster http://127.0.0.1:8899
anchor deploy --provider.cluster http://127.0.0.1:8899
when deployed, copy the programId and paste it inside declareid!() and Anchor.toml (you can get programId from "anchor keys list" too)

execute test validator on ubuntu root: solana-test-validator --reset

anchor build --provider.cluster http://127.0.0.1:8899
solana airdrop 10 56BYySo1zyozEWkHEyvF1cFG6PhcKAmnEvSWvFJqW6W6 --url http://127.0.0.1:8899
anchor deploy --provider.cluster http://127.0.0.1:8899
