package proxy

import (
	"github.com/charmbracelet/log"
	"github.com/gin-gonic/gin"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"time"
)

type Proxy struct {
	TargetURL *url.URL

	proxy  *gin.Engine
	webui  *gin.Engine
	logger *log.Logger
}

func NewProxy(target string) (*Proxy, error) {
	targetURL, err := url.Parse(target)
	if err != nil {
		return nil, err
	}

	gin.SetMode(gin.ReleaseMode)
	r := &Proxy{
		TargetURL: targetURL,
		proxy:     gin.New(),
		webui:     gin.New(),
		logger: log.NewWithOptions(os.Stdout, log.Options{
			ReportCaller:    false,
			ReportTimestamp: true,
			TimeFormat:      time.Kitchen,
			Prefix:          "🔍 ",
		}),
	}

	r.proxy.Use(gin.Recovery())
	r.proxy.Any("/*proxyPath", r.ProxyHandlerFunc())
	r.webui.Use(gin.Recovery())
	r.webui.GET("/", r.WebuiHandlerFunc())

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
		defer resp.Body.Close()

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
	}
}

func (p *Proxy) WebuiHandlerFunc() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": http.StatusOK})
	}
}

func (p *Proxy) Run(addr string) {
	stop := make(chan struct{})

	go func() {
		srv := &http.Server{Addr: addr, Handler: p.proxy}
		p.logger.Infof("Proxy server started at %s", addr)
		if err := srv.ListenAndServe(); err != nil {
			p.logger.Fatalf("Failed to start proxy server: %v", err)
		}
	}()

	go func() {
		listener, err := net.Listen("tcp", "127.0.0.1:0")
		if err != nil {
			p.logger.Errorf("Failed to start webui server: %v", err)
			return
		}
		defer listener.Close()
		webuiAddr := listener.Addr().String()

		srv := &http.Server{Addr: webuiAddr, Handler: p.webui}
		p.logger.Infof("Webui server started at %s", webuiAddr)
		if err := srv.Serve(listener); err != nil {
			p.logger.Fatalf("Failed to start webui server: %v", err)
		}
	}()

	<-stop
}
