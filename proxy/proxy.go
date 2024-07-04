package proxy

import (
	"github.com/charmbracelet/log"
	"github.com/gin-gonic/gin"
	"io"
	"net/http"
	"net/url"
	"os"
	"time"
)

type Proxy struct {
	*gin.Engine
	TargetURL *url.URL

	logger *log.Logger
}

func NewProxy(target string) (*Proxy, error) {
	targetURL, err := url.Parse(target)
	if err != nil {
		return nil, err
	}

	gin.SetMode(gin.ReleaseMode)
	r := &Proxy{
		Engine:    gin.New(),
		TargetURL: targetURL,
		logger: log.NewWithOptions(os.Stdout, log.Options{
			ReportCaller:    false,
			ReportTimestamp: true,
			TimeFormat:      time.Kitchen,
			Prefix:          "🔍 ",
		})}
	r.Use(gin.Recovery())
	r.Any("/*proxyPath", r.ProxyHandlerFunc())

	return r, nil
}

func (p *Proxy) ProxyHandlerFunc() gin.HandlerFunc {
	return func(c *gin.Context) {
		proxyURL := p.TargetURL.ResolveReference(c.Request.URL)
		req, err := http.NewRequest(c.Request.Method, proxyURL.String(), c.Request.Body)
		if err != nil {
			p.logger.Errorf("Failed to create proxy request: %v", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create request"})
			return
		}
		req.Header = c.Request.Header

		client := &http.Client{}
		resp, err := client.Do(req)
		if err != nil {
			p.logger.Errorf("Failed to send proxy request: %v", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to send proxy request"})
			return
		}

		p.logger.Info("Request proxied", "url", proxyURL.String(), "statusCode", resp.StatusCode)
		for key, values := range resp.Header {
			for _, value := range values {
				c.Writer.Header().Add(key, value)
			}
		}
		c.Status(resp.StatusCode)
		_, err = io.Copy(c.Writer, resp.Body)
		if err != nil {
			p.logger.Errorf("Failed to copy response body: %v", err)
			return
		}

		err = resp.Body.Close()
		if err != nil {
			p.logger.Errorf("Failed to close response body: %v", err)
			return
		}
	}
}

func (p *Proxy) Run(addr string) error {
	p.logger.Infof("Listening and serving proxy on %s", addr)
	return p.Engine.Run(addr)
}
