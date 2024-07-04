package db

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"net/http"
)

const inMemoryConnection = "file::memory:?cache=shared"

type DB struct {
	*gorm.DB
}

type Request struct {
	gorm.Model
	RequestURI string `form:"path" json:"path" binding:"required"`
}

type Response struct {
	gorm.Model
	RequestID string
}

func NewDB() (*DB, error) {
	db, err := gorm.Open(sqlite.Open(inMemoryConnection), &gorm.Config{})
	if err != nil {
		return nil, err
	}

	db.AutoMigrate(&Request{})
	return &DB{db}, nil
}

func (db *DB) InsertRequest(req *http.Request) error {
	return nil
}

func (db *DB) FindRequest(reqID string) (*Request, error) {
	return nil, nil
}

func (db *DB) DeleteRequest(reqID string) (*Request, error) {
	return nil, nil
}
