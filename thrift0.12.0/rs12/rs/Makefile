
run:
	cargo run --bin server

client:
	cargo run --bin client

gen:
	thrift12 -out src --gen rs msg.thrift


push:
	git add .
	git commit -m 'pu'
	git push
