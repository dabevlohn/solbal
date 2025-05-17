url := 'https://grpc.ny.shyft.to'
token := 'b2b972c6-fff2-4b5c-aac9-375c6984b80e'
pkey := 'Ac4R6EFdjkNaVBoUk4T26XrGmdhQTdHcwpswCqopfbS6'
#pkey := 'CRnkKQTxctQ7LHVN3yssdgJyEksBJeBrDdwZAxBtsJoZ'

# Get balances
bal:
	RUST_LOG=info cargo run --bin balances

# Get block height
block:
	RUST_LOG=info cargo run --bin client -- \
		-e {{url}} --x-token {{token}} get-block-height

# Subscribe
sub:
	RUST_LOG=info cargo run --bin client -- \
		-e {{url}} --x-token {{token}} subscribe \
		--blocks-account-include {{pkey}}
