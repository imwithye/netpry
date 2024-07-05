package main

import (
	"fmt"
	"log"
	"net/url"

	"github.com/alecthomas/kingpin/v2"
	"github.com/imwithye/netpry/proxy"
)

var (
	proxyPort   = kingpin.Arg("port", "Port to listen on.").Required().Int()
	proxyTarget = kingpin.Arg("target", "Reverse proxy target.").Required().String()

	proxyHost = kingpin.Flag("host", "Host to listen on.").Default("127.0.0.1").String()

	webuiHost = kingpin.Flag("dashboard-host", "Host to listen on for the webui.").Default("127.0.0.1").String()
	webuiPort = kingpin.Flag("dashboard-port", "Port to listen on for the webui.").Default("0").Int()
)

func Value[T any](v *T) T {
	if v == nil {
		var defaultValue T
		return defaultValue
	}
	return *v
}

func Parse() (*proxy.Proxy, error) {
	kingpin.Parse()

	proxyTargetURL, err := url.Parse(Value(proxyTarget))
	if err != nil {
		return nil, err
	}

	proxyAddr := fmt.Sprintf("%s:%d", Value(proxyHost), Value(proxyPort))
	webuiAddr := fmt.Sprintf("%s:%d", Value(webuiHost), Value(webuiPort))
	return proxy.NewProxy(proxyTargetURL, proxyAddr, webuiAddr)
}

func main() {
	proxy, err := Parse()
	if err != nil {
		log.Fatalln(err)
		return
	}
	proxy.Run()
}
