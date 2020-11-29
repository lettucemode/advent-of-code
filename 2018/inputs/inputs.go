package inputs

import (
	"fmt"
	"io/ioutil"
	"path"
	"runtime"
)

// GetInput ...
func GetInput(day int) (ret string) {
	_, filename, _, _ := runtime.Caller(0)
	dat, _ := ioutil.ReadFile(fmt.Sprintf(path.Dir(filename) + "/d%v.txt", day))
	ret = string(dat)
	return
}