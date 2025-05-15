# Get balances
bal:
	cargo run --bin balances

# Get block height
block:
	cargo run --bin client -- -e "https://grpc.ny.shyft.to" --x-token "b2b972c6-fff2-4b5c-aac9-375c6984b80e" get-block-height
