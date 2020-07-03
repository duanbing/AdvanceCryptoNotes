# LSSS

[toc]

## 介绍

线性秘钥共享是在素域$F_q$上构造的秘钥共享方案。是shamir秘钥共享的一般形式。

### 算法基本原理

已知矩阵 $M \in F_q^{m \times d} $ 和向量 $v \in F_q^d$, 以及映射 $ x: \{1, ...,m\} \rightarrow \{1, ..., n\}$. 为了分享秘密 $s \in F_q$,  我们选择向量 $k \in F_q^d$

, 使得下面等式成立：


$$
s = v^T \cdot k
$$
同时计算：
$$
s = (s_1, ..., s_m)^T = M \cdot k
$$
其中对于参与方j, 如果x(i) = j,  获得秘钥分片$s_i$ 



一个参与方集合 A 可以恢复秘钥，必须满足：矩阵M的所有 **行i**（其中$ x(i) = j, j \in A $ ）张成(span)构成的空间包含向量v。即存在$x_A$使得：
$$
v^T = x_A^T \cdot M_A \\
$$
根据$s_A = M_A \cdot k$ , 也就是加密之后的秘密值， 可以推导出：
$$
x_A^T \cdot s_A = x_A^T \cdot (M_A \cdot k) =(x_A^T \cdot M_A)\cdot k = v^T \cdot k = s.
$$
对于Shamir共享，选择矩阵M和向量v如下：
$$
M = \begin{pmatrix} 1 & 1 & ... & 1^t \\ 1 & 2 & ... & 2^t \\. & & & .\\1 & n & n & n^t\end{pmatrix}
$$
对于所有的i，满足 x(i) = i。 

### 例子

 例如一份文件，只有开发部门的A项目组的测试人员可看， 这一组属性就可以分解为一个属性集合[开发部门， A项目组， 测试人员], 只有同时满足这个属性集合的所有条件才能查看该会议文件。 分别用 $P_a, P_b, P_c$表示开发部门，A项目组和测试人员。 P表示访问策略。可知： $ P = P_a \wedge P_b \wedge P_c$。

### 属性转换为分享矩阵

#### 定义

​     让每个属性表示为 $M_U \in Z ^ {1 \times 1}$类型的但条目矩阵， 即： $M_U = [1]$. 存在矩阵 $M_a\in Z^{d_a \times e_a}$表示访问策略$P_a$, 用$C_a\in Z^{e}$表示矩阵$M_a$的第一列， 用$R_a \in Z^{(d_a - 1) \times e_a}$表示矩阵$M_a$除去第一列的剩余列： 存在矩阵$M_b \in Z ^ {d_b \times e_b}$ 表示访问策略$P_b$, 用$C_b$表示矩阵$M_b$的第一列，$R_b$表示矩阵$M_b$除去第一列的剩余列。 

#### 规则

* 规则1 访问策略P的每个变量$a_i$都可以用矩阵$M_U$表示；

* 规则2 对于任意的或结构(OR-term), $P = P_a \vee P_b$, 可以用矩阵 $ M_{OR} \in Z ^ {(d_a + d_b)\times (e_a + e_b - 1)}$来表示访问策略P，形式如下:
  $$
  M_{OR} = \begin{pmatrix} C_a & R_a & 0 \\ C_b & 0 & R_b\end{pmatrix}
  $$

* 规则3 对于任意与结构(AND-term)， $ P = P_a \wedge P_b$, 我们可以用矩阵$M_{AND} \in Z^{(d_a + d_b) \times (e_a + e_b)}$来表示访问策略P,形式如下：
  $$
  M_{AND} = \begin{pmatrix} C_a & C_a & R_a & 0 \\ 0 & C_b & 0 & R_b\end{pmatrix}
  $$

经过上面的几个规则，可以获得矩阵M。



### 求解过程

我们选择分享矩阵  $\rho  = (s, \rho_2, ..., \rho_e)^T$,  $\rho_i$是一组随机数，s是秘密加密过程如下：
$$
M \cdot \rho = (s_1, .., s_d)^T
$$
对于解密过程，假设存在目标向量， $\epsilon = (1, 0, ..., 0)^T \in Z^e$, 可以使$M_A^T \cdot x_A = \epsilon$, 使得
$$
x_A^T \cdot s_A = x_A^T \cdot (M_A \cdot \rho) =(x_A^T \cdot M_A)\cdot \rho = \epsilon^T \cdot \rho = s.
$$
其中求解$x_A$可以使用高斯消元法。

实际例子可以参考2.

## LSSS性质

### 非交互性

假设s 和s' 通过 $s = M \cdot k, s^{'} = M \cdot k^{'}$ 进行分享，可以推出：

1. s + s'  可以通过  $s + s' = M \cdot (k + k')$,  同时 $s + s' = v^T \cdot (k + k')$ 
2. $\alpha \cdot s$ 可以通过$\alpha \cdot s = M \cdot (\alpha \cdot k)$进行分享 

## Maurer’s Multiplication Protocol

Maurer乘法协议相对Beaver方法和Damgård-Nielsen方法不需要做额外的预处理, 并且计算过程节点之间交互较少。 另外从参考3知道， Schur product满足交换律，结合律以及分配律。

对于s和s'的分片向量 $ (s_1,..., s_n)$ 和 $(s_1^{'}, ... , s_n^{'})$， 存在矩阵 $(v_1, ..., v_n)$, $v_i$是一个向量， 使得：
$$
|v_i| = |s_i \bigotimes s_j^{'}|  \\
s \cdot s' = \sum_{i=1}^{n} v_i \cdot (s_i \bigotimes s_j^{'})
$$
例如这个$\bigotimes$可以是Schur product的线性和。

例如： 对于 $ s = k_1 + k_2 + k_3$,  计算参与方1 拥有$(k_2, k_3)$,  参与方2拥有 $(k_1, k_3)$， 参与方3 拥有 $(k_1, k_2)$， 可以获得：
$$
\begin{align}
s \cdot s' &= (k_1 + k_2 + k_3)\cdot(k_1^{'} + k_2^{'} + k_3^{'})  \\
 &= (k_2 * k_2^{'} + k_2 * k_3^{'}  + k_3 * k_2^{'} ) \\
    &+ (k_3 * k_3^{'} + k_1 * k_3^{'}  + k_3 * k_1^{'} ) \\
    &+ (k_1 * k_1^{'} + k_1 * k_2^{'}  + k_2 * k_1^{'} ) \\
 \end{align}
$$


为保证更好安全等级下的计算安全， 在(n, t)阈值模型下，t 满足： t < n/3.



假设

因此，Maurer’s Multiplication Protocol步骤如下：

1. 每个参与方计算各自拥有的分片的Schur product； 
2. 将第一步计算的Schur product的和加起来； 例如12, 13, 14步的计算；
3. 参与方通过full threshhold sharing(例如shamir sharing)再次讲第二步的结果进行分享；
4. 各方都计算$s \cdot s'$,  并且reconstruct。 



## 参考

1. https://homes.esat.kuleuven.be/~nsmart/FHE-MPC/LSSS-MPC.pdf
2. https://blog.csdn.net/ping802363/article/details/77900273
3. [https://zh.wikipedia.org/wiki/%E9%98%BF%E9%81%94%E7%91%AA%E4%B9%98%E7%A9%8D_(%E7%9F%A9%E9%99%A3)](https://zh.wikipedia.org/wiki/阿達瑪乘積_(矩陣))
4. https://www.iacr.org/archive/eurocrypt2000/1807/18070321-new.pdf  General Secure Multi-Party Computation from any Linear Secret-Sharing Scheme

