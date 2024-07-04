package main

import (
	"github.com/imwithye/netpry/proxy"
	"log"
)

func main() {
	p, err := proxy.NewProxy("https://google.com")
	if err != nil {
		log.Fatalln(err)
		return
	}
	err = p.Run(":9080")
	if err != nil {
		log.Fatalln(err)
		return
	}
}
