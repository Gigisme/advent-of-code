package utils

import (
	"bufio"
	"io"
	"os"
)

type LineReader struct {
	file    *os.File
	scanner *bufio.Scanner
}

func NewLineReader(filepath string) (*LineReader, error) {
	file, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}

	return &LineReader{
		file:    file,
		scanner: bufio.NewScanner(file),
	}, nil
}

func (lr *LineReader) NextLine() (string, error) {
	if lr.scanner.Scan() {
		return lr.scanner.Text(), nil
	}
	if err := lr.scanner.Err(); err != nil {
		return "", err
	}
	return "", io.EOF // End of file
}

func (lr *LineReader) ReadAllLines() ([]string, error) {
	var lines []string
	for {
		line, err := lr.NextLine()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, err
		}
		lines = append(lines, line)
	}
	return lines, nil
}

func (lr *LineReader) Close() error {
	return lr.file.Close()
}
