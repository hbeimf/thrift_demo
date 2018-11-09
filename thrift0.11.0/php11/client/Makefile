run:client
	php client.php

client:
	rm -rf ./msg
	thrift11 -r --gen php msg.thrift
	mv ./gen-php/msg ./
	rm -rf ./gen-php

out:
	thrift11 -out xxx -r --gen php msg.thrift 
