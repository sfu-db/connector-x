echo "user: $USER"
pgid=$(ps -o pgid,comm -u $USER | grep just | awk '{print $1}')
echo "pgid of command: $pgid"
max=0

for((i=0;i<1000000;i++))
do
    ps -o pid,ppid,pgid,comm,rss -u $USER | grep $pgid
    sum=$(ps -o pid,ppid,pgid,comm,rss -u $USER | grep $pgid | awk '{sum += $NF} END {print sum}')
    echo "current sum: $sum"
    if (( sum > max )); then
        max=$sum
    fi
    echo "current max: $max"
    [ -z $sum ] && exit 0 || echo "continue..."
    sleep 2
done
