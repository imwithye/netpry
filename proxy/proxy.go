package proxy

import (
	"github.com/gin-gonic/gin"
	"io"
	"log"
	"net/http"
	"net/url"
)

type Proxy struct {
	*gin.Engine
	TargetURL *url.URL
}

func NewProxy(target string) (*Proxy, error) {
	targetURL, err := url.Parse(target)
	if err != nil {
		return nil, err
	}

	gin.SetMode(gin.ReleaseMode)
	r := &Proxy{Engine: gin.New(), TargetURL: targetURL}
	r.Use(gin.Recovery())
	r.Any("/*proxyPath", r.ProxyHandlerFunc())

	return r, nil
}

func (p *Proxy) ProxyHandlerFunc() gin.HandlerFunc {
	return func(c *gin.Context) {
		proxyURL := p.TargetURL.ResolveReference(c.Request.URL)
		req, err := http.NewRequest(c.Request.Method, proxyURL.String(), c.Request.Body)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create request"})
			return
		}
		req.Header = c.Request.Header
		client := &http.Client{}
		resp, err := client.Do(req)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to send request"})
			return
		}
		log.Println("Request proxied to", proxyURL.String(), resp.StatusCode)
		for key, values := range resp.Header {
			for _, value := range values {
				c.Writer.Header().Add(key, value)
			}
		}
		c.Status(resp.StatusCode)
		_, err = io.Copy(c.Writer, resp.Body)
		if err != nil {
			log.Println("Failed to copy response body")
			return
		}
		err = resp.Body.Close()
		if err != nil {
			log.Println("Failed to close response body")
			return
		}
	}
}

func (p *Proxy) Run(addr string) error {
	log.Println("Listening and serving proxy on", addr)
	return p.Engine.Run(addr)
}
