package proxy

import (
	"net"
	"net/http"
	"net/url"
	"os"
	"time"

	"github.com/charmbracelet/log"
	"github.com/gin-gonic/gin"
	"github.com/imwithye/netpry/db"
)

type Proxy struct {
	proxyTargetURL *url.URL
	proxyAddr      string
	webuiAddr      string

	db     *db.DB
	proxy  *gin.Engine
	webui  *gin.Engine
	logger *log.Logger
}

func NewProxy(proxyTargetURL *url.URL, proxyAddr, webuiAddr string) (*Proxy, error) {
	database, err := db.NewDB()
	if err != nil {
		return nil, err
	}

	gin.SetMode(gin.ReleaseMode)
	r := &Proxy{
		proxyTargetURL: proxyTargetURL,
		proxyAddr:      proxyAddr,
		webuiAddr:      webuiAddr,

		db:    database,
		proxy: gin.New(),
		webui: gin.New(),
		logger: log.NewWithOptions(os.Stdout, log.Options{
			Level:           log.DebugLevel,
			ReportCaller:    false,
			ReportTimestamp: true,
			TimeFormat:      time.Kitchen,
			Prefix:          "🔍 ",
		}),
	}

	r.proxy.Use(r.logMiddleware("proxy"), gin.Recovery())
	r.proxy.Any("/*proxyPath", r.proxyHandler())
	r.webui.Use(r.logMiddleware("webui"), gin.Recovery())
	r.webui.GET("/*webuiPath", r.webuiHandler())

	return r, nil
}

func (p *Proxy) Run() {
	go func() {
		listener, err := net.Listen("tcp", p.webuiAddr)
		if err != nil {
			p.logger.Errorf("Failed to start webui server: %v", err)
			return
		}
		defer listener.Close()
		webuiAddr := listener.Addr().String()

		srv := &http.Server{Addr: webuiAddr, Handler: p.webui}
		p.logger.Infof("Webui server started at http://%s", webuiAddr)
		if err := srv.Serve(listener); err != nil {
			p.logger.Fatalf("Failed to start webui server: %v", err)
		}
	}()

	listener, err := net.Listen("tcp", p.proxyAddr)
	if err != nil {
		p.logger.Errorf("Failed to start proxy server: %v", err)
		return
	}
	defer listener.Close()
	proxyAddr := listener.Addr().String()

	srv := &http.Server{Addr: proxyAddr, Handler: p.proxy}
	p.logger.Infof("Proxy http://%s --> %s", proxyAddr, p.proxyTargetURL.String())
	if err := srv.Serve(listener); err != nil {
		p.logger.Fatalf("Failed to start proxy server: %v", err)
	}
}
