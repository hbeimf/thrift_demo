run:client
	php client.php

client:
	rm -rf ./msg
	thrift -r --gen php msg.thrift
	mv ./gen-php/msg ./
	rm -rf ./gen-php
