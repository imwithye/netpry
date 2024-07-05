package proxy

import "github.com/gin-gonic/gin"

func (p *Proxy) logMiddleware(prefix string) gin.HandlerFunc {
	return func(c *gin.Context) {
		p.logger.Infof("[%s] %s - %s", prefix, c.Request.Method, c.Request.URL.String())
		c.Next()
	}
}
