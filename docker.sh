#!/bin/bash
PROJECT_HOME=$HOME/github/cs453-project
docker run --rm -it --security-opt seccomp=unconfined -v $PROJECT_HOME:/usr/src tarpaulin /bin/bash
