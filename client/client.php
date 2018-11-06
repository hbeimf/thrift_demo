<?php
include 'vendor/autoload.php';

use Thrift\Protocol\TBinaryProtocol;
use Thrift\Transport\TBufferedTransport;
use Thrift\Transport\TSocket;

require_once dirname(__FILE__) . '/msg/MsgService.php';
require_once dirname(__FILE__) . '/msg/Types.php';

class ThriftClient {
	function call($cmd, $text) {
		$msg = new \msg\Message(['id' => $cmd, 'text' => $text]);
		return $this->client->hello($msg);
	}
	function __construct() {
		$socket = new TSocket($this->_host, $this->_port);
		$this->transport = new TBufferedTransport($socket, 1024, 1024);
		$protocol = new TBinaryProtocol($this->transport);
		$this->client = new \msg\MsgServiceClient($protocol);
		$this->transport->open();
	}
	function __destruct() {
		$this->transport->close();
	}
	public $transport = null;
	public $client = null;
	private $_host = "localhost";
	private $_port = 9090;
}

$client = new ThriftClient();
$r = $client->call(123, "hello world");

print_r($r);

?>