run:gen
	php client.php

gen:
	rm -rf ./msg
	thrift11 -r --gen php msg.thrift
	mv ./gen-php/msg ./
	rm -rf ./gen-php

gen123:
	thrift11 -out xxx -r --gen php msg.thrift 
