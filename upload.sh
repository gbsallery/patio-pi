#!/bin/bash
curl -v http://192.168.1.183:8000/image --data-binary @$1 -H "Content-Type: application/octet-stream"
