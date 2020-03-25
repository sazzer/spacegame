package database

import (
	"io"
	"os"

	"github.com/golang-migrate/migrate/v4/source"
	"github.com/phogolabs/parcello"
	"github.com/sirupsen/logrus"
)

type parcelloSource struct {
	fileSystem parcello.FileSystemManager
	files      []string
}

func newParcelloSource() parcelloSource {
	fileSystem := parcello.ManagerAt("/")
	files := []string{}
	fileSystem.Walk("/", func(path string, info os.FileInfo, err error) error {
		if !info.IsDir() {
			logrus.WithField("path", path).WithField("isDir", info.IsDir()).Trace("Found file")
			files = append(files, path)
		}
		return nil
	})
	logrus.WithField("files", files).Debug("Found files")

	return parcelloSource{fileSystem: fileSystem, files: files}
}

func (p parcelloSource) Open(url string) (source.Driver, error) {
	logrus.Debug("Opening Source")
	return p, nil
}

func (p parcelloSource) Close() error {
	logrus.Debug("Closing Source")
	return nil
}

func (p parcelloSource) First() (uint, error) {
	logrus.WithField("total", len(p.files)).Debug("Looking for First migration")
	if len(p.files) == 0 {
		return 0, os.ErrNotExist
	}
	return 0, nil
}

func (p parcelloSource) Prev(version uint) (uint, error) {
	return 0, os.ErrNotExist
}

func (p parcelloSource) Next(version uint) (uint, error) {
	total := uint(len(p.files))
	logrus.WithField("version", version).WithField("total", total).Trace("Looking for Next migration")
	if total > version+1 {
		logrus.WithField("version", version).WithField("total", total).Debug("Found Next migration")
		return version + 1, nil
	}
	logrus.WithField("version", version).WithField("total", total).Debug("No Next migration")
	return 0, os.ErrNotExist
}

func (p parcelloSource) ReadUp(version uint) (io.ReadCloser, string, error) {
	total := uint(len(p.files))
	logrus.WithField("version", version).WithField("total", total).Trace("Reading Up Migration")
	if total > version {
		filename := p.files[version]
		logrus.WithField("version", version).WithField("filename", filename).Debug("Reading Migration file")

		file, err := p.fileSystem.Open(filename)
		if err != nil {
			logrus.WithField("version", version).WithField("filename", filename).Error("Error reading Migration file")
			return nil, filename, err
		}
		return file, filename, nil
	}
	return nil, "", os.ErrNotExist
}

func (p parcelloSource) ReadDown(version uint) (io.ReadCloser, string, error) {
	return nil, "", os.ErrNotExist
}
