```bash
go test -v 2>&1 ./examples/n3/... -run Test -bench Benchmark | go-junit-report -set-exit-code > ./examples/n3/report.xml
```