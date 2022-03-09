<?php

	echo "Hello World {$_SERVER['HTTP_HOST']}\nFrom Rust Reverse Proxy!";

	print_r($_SERVER);

	echo "\n";

	print_r(getallheaders());

?>
