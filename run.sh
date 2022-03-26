#!/bin/bash
export OUTPUT_PATH=files/output
echo "Code logs"
echo "============================="
cat files/input | node $1
echo "============================="
echo "Output"
echo "============================="
echo $(cat files/output)