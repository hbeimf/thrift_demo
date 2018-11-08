
run:
	rebar3 shell

# client:
# 	cargo run --bin client

# gen:
# 	thrift11 -out src --gen erl msg.thrift

gen:
	thrift11 --gen erl msg.thrift
	cp ./gen-erl/*.hrl ./apps/rpc_server/include/ 
	cp ./gen-erl/*.erl ./apps/rpc_server/src/ 

