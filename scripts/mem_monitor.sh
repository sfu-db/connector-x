for((i=0;i<1000000;i++))
do
        for pid in $(sudo pidof -c $PWD python)
        do
		if [[ $(stat -c "%u" /proc/$pid/) == $(id -u) ]]
		then
                    echo -n $pid " "
                    cat /proc/$pid/status | grep VmHWM
		fi
        done
        echo "===="
        sleep 2
done
