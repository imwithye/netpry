package proxy

import (
	"embed"
	"io"
	"io/fs"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

//go:embed _webui/dist
var embedWebuiFS embed.FS

func (p *Proxy) webuiHandler() gin.HandlerFunc {
	subFS, _ := fs.Sub(embedWebuiFS, "_webui/dist")
	httpFS := http.FileServer(http.FS(subFS))
	return func(c *gin.Context) {
		fp := strings.TrimPrefix(c.Request.URL.Path, "/")
		req := c.Request.Clone(c.Request.Context())
		_, err := subFS.Open(fp)
		if err != nil {
			req.URL.Path = "/"
		}
		httpFS.ServeHTTP(c.Writer, req)
	}
}

func (p *Proxy) proxyHandler() gin.HandlerFunc {
	return func(c *gin.Context) {
		proxyURL := p.proxyTargetURL.ResolveReference(c.Request.URL)
		req, err := http.NewRequest(c.Request.Method, proxyURL.String(), c.Request.Body)
		if err != nil {
			p.logger.Errorf("Failed to create proxy request: %v", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create request"})
			return
		}
		req.Header = c.Request.Header

		client := NewHttpClient()
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
