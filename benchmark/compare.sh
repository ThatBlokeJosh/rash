for i in {1..99}
do
	for j in {1..99}
	do
		for k in {1..99}
		do
			echo $(($i * $j * $k))
		done
	done
done
