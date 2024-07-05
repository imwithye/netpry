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
	TargetURL *url.URL

	db     *db.DB
	proxy  *gin.Engine
	webui  *gin.Engine
	logger *log.Logger
}

func NewProxy(target string) (*Proxy, error) {
	targetURL, err := url.Parse(target)
	if err != nil {
		return nil, err
	}

	database, err := db.NewDB()
	if err != nil {
		return nil, err
	}

	gin.SetMode(gin.ReleaseMode)
	r := &Proxy{
		TargetURL: targetURL,

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

func (p *Proxy) Run(addr string) {
	stop := make(chan struct{})

	go func() {
		listener, err := net.Listen("tcp", addr)
		if err != nil {
			p.logger.Errorf("Failed to start proxy server: %v", err)
			return
		}
		defer listener.Close()
		proxyAddr := listener.Addr().String()

		srv := &http.Server{Addr: proxyAddr, Handler: p.proxy}
		p.logger.Infof("Proxy server started at http://%s", proxyAddr)
		if err := srv.Serve(listener); err != nil {
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
		p.logger.Infof("Webui server started at http://%s", webuiAddr)
		if err := srv.Serve(listener); err != nil {
			p.logger.Fatalf("Failed to start webui server: %v", err)
		}
	}()

	<-stop
}
