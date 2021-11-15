package main

import (
	"fmt"
	"math/rand"
	"os"
	"time"
)

func main() {
	ping()
	//TODO Handle and log errors from echo() inside main() and continue.
	echo("Hello, world!")
	//TODO If there are jobs, print them, else print nothing.
	job_count()
	//TODO Lastly, move these functions outside of main.rs so we can reuse and test them.
}

//TODO fmt.Println or writing to stderr should happen in main()
func ping() {
	is_ok := is_successful()
	if is_ok {
		fmt.Println("UP")
	} else {
		os.Stderr.WriteString("ERROR: DOWN\n")
	}
}

//TODO fmt.Println or writing to stderr should happen in main()
func echo(m string) {
	is_ok := is_successful()
	if is_ok {
		fmt.Println(m)
	} else {
		//TODO This kills program execution, which we don't want.
		panic("ERROR: We encountered error code 42")
	}
}

//TODO fmt.Println or writing to stderr should happen in main()
func job_count() {
	exists := is_successful()
	if exists {
		r := rand.New(rand.NewSource(time.Now().UnixNano()))
		fmt.Println(r.Uint32())
	} else {
		fmt.Println("nil")
	}
}

/// Just a dummy function to simulate unexpected behavior.
/// `./main --skip` to always run happy path.
func is_successful() bool {
	for _, arg := range os.Args[1:] {
		if arg == "--skip" {
			return true
		}
	}
	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	return r.Uint32() > 2147483647
}
