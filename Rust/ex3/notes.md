# Exercise 6

distance: d, time pressed: n, time total: t  
$ d = (t-n) * n $

smalles n for at leat distance d  
$ d = tn - n^2 \equiv f(x) = px + qx^2 $  
$ 0 = -n^2 + tn - d = n^2 - tn + d \implies p = -t, q = d $  
$ x_{1,2} = -\frac{p}{2} \pm \sqrt{ (\frac{p}{2})^2 - q } $  
$ \implies n_{1,2} = \frac{t}{2} \pm \sqrt{ \frac{t^2}{4} - d } $

example:  
$ t=7, d=9 \implies n_1=5.3, n_2=1.6 $  
$ \implies min = \text{ceil}(n_2), max = \text{floor}(n_1) $  