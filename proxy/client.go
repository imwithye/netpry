package proxy

import (
	"net/http"
	"time"
)

type HttpClient struct {
	*http.Client
}

func NewHttpClient() *HttpClient {
	transport := &http.Transport{
		MaxIdleConns:        128,
		IdleConnTimeout:     120 * time.Second,
		DisableKeepAlives:   false,
		MaxIdleConnsPerHost: 32,
	}
	httpClient := &http.Client{
		Transport: transport,
	}
	return &HttpClient{httpClient}
}
