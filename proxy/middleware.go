package proxy

import "github.com/gin-gonic/gin"

func (p *Proxy) logMiddleware(prefix string, useDebug bool) gin.HandlerFunc {
	return func(c *gin.Context) {
		logfFn := p.logger.Infof
		if useDebug {
			logfFn = p.logger.Debugf
		}
		logfFn("[%s] %s - %s", prefix, c.Request.Method, c.Request.URL.RequestURI())
		c.Next()
	}
}
