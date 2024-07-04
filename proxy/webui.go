package proxy

import (
	"embed"
	"github.com/gin-gonic/gin"
	"io/fs"
	"net/http"
	"strings"
)

//go:embed _webui/dist
var embedWebuiFS embed.FS

func (p *Proxy) webuiHandler() gin.HandlerFunc {
	subFS, _ := fs.Sub(embedWebuiFS, "_webui/dist")
	return func(c *gin.Context) {
		fp := strings.TrimPrefix(c.Request.URL.Path, "/")
		req := c.Request.Clone(c.Request.Context())
		_, err := subFS.Open(fp)
		if err != nil {
			req.URL.Path = "/"
		}
		fs := http.FileServer(http.FS(subFS))
		fs.ServeHTTP(c.Writer, req)
	}
}
