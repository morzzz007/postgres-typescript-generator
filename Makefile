SHELL=/bin/bash
.PHONY: help publish

default: help

help: ## This help message
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' -e 's/:.*#/: #/' | column -t -s '##'

publish:
	./build-and-publish.sh
