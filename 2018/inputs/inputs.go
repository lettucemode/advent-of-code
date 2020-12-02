package inputs

import (
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"path"
	"runtime"
	"strings"
)

// GetInput ...
func GetInput(day int) io.Reader {
	_, filename, _, _ := runtime.Caller(0)
	dat, err := ioutil.ReadFile(fmt.Sprintf(path.Dir(filename) + "/d%v.txt", day))
	if err != nil {
		log.Fatal(err)
	}
	return strings.NewReader(string(dat))
}