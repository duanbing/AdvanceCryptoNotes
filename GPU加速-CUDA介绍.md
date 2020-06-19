[toc]

## GPU加速-CUDA介绍[^1]

​	CUDA是Nvidia提供的一套针对GPU设备的并行编程平台。支持多款GPU设备以及多种编程语言套件进行应用开发。GPU计算通常采用CPU+GPU异构模式。由CPU负责执行复杂逻辑处理和事务处理等不适合数据并行的数据，由GPU负责计算密集型的大规模数据并行计算。

​	CUDA的软件栈分为3层： CUDA Library, CUDA runtime API, CUDA driver API.  Runtime API是driver API的封装，Library 是例如cuDNN、cuFFT等之类的库。

CUDA C语言为程序提供了一种用C语言编写设备端代码的编程方式，包括对C的一些必要扩展和一个运行时库。CUDA对C的扩展主要包括以下几个方面: 

​     1.引入了函数类型限符。用来规定函数是在host还是在device上执行，以及这个函数是从host调用还是从device调用。这些限定符有：\_\_device\_\_,\__host\_\_,\_\_global\_\_.

​     2.引入了变量类型限定符。用来规定变量被存储在哪一类存储器上。传统的在CPU上运行的程序，编译器能自动决定将变量存储在CPU的寄存器还是内存中，在CUDA编程模型中，一共抽象出来多棕8种不同的存储器。为了区分各种存储器，必须引入一些限定符，包括：\_\_device\_\_,_\_shared\_\_和\_\_constant\_\_。注意，此处的\_\_device\_\_与上节中的\_\_device\_\_限定符的含义不同。

​    3.引入了内置变量类型。如char4,ushort3,double2,dim3等，它们是由基本的整型可浮点型构成的矢量类型，通过x,y,z,w访问每一个分量，在设备端代码中各矢量类型有不同的对齐要求。

​    4.引入了4个内建变量。blockIdx和threadIdx用于索引线程块和线程，gridDim和blockDim用于描述线程网格和线程块的维度。warpZize用于查询warp中的线程数量。

​    5.引入了<<< >>>运算符。用于指定线程网格和线程维度，传递执行参数。

​    6.引入了一些函灵敏：memory fence函数，同步函数，数学函数，纹理函数，测时函数，原子函数，warp vote函数。

​	CUDA runtime API和CUDA driver API提供了实现设备管理（Device management)，上下文管理（Context management），存储器管理费用（Memory Control），代码块管理 （Code Module management），执行控制（Excution Control），纹理索引管理（Texture Reference management）与OpenGL和Direct3D的互操作性（Interoperity with OpenGL and Direct3D）的应用程序接口。

   CUDA runtime API在CUDA driver API 的基础上进行了封装，隐藏了一些实现细节，编程更加方便，代码更加简洁。CUDA runtime API被打包放在CUDAArt包里，其中的函数都有CUDA 前缀。CUDA运行时没有专门的初始化函数，它将在第一次调用函数时自动完成初始化。对使用运行时函数的CUDA程序测试时要避免将这段初始化的时间计入。CUDA runtime API的编程较为简洁，通常都会用这种API进行开发。

   CUDA driver API是一种基于句柄的底层接口（式多对象通过句柄被引用），可以加载二进制或汇编形式的内核函数模块，指定参数，并启动计算。CUDA driver API的编程复杂，但有时能通过直接操作硬件的执行实行一些更加复杂的功能键，或者获得更高的性能。由于它使用的设备端代码是二进制或者汇编代码，因此可以在各种语言中调用。CUDA driver API被放在nvCUDA包里，所有函数前缀为cu。

​	其中文档[^3]列出所有目前cuda的不同版本的之类情况。

### 概念介绍

SM， Stream Multiprocessor:  流多处理器，每个GPU卡由多个SM组成， 一个SM由多个SP, Scalar Processor组成, SP包含基本的执行单元和分支预测单元。

Kernels:  一组并行运行在设备上的函数，以`__globla__`修饰。一个Kernel的所有线程， 分为Grid、Block以及Thread三层逻辑关系。如下：

![Grid of Thread Blocks.](https://docs.nvidia.com/cuda/cuda-c-programming-guide/graphics/grid-of-thread-blocks.png)

一个Grid由多个Block组成，使用内置变量blockIdx表示其位置， 一个Block由多个Thread组成，每个Thread使用threadIdx表示其位置。

​	Thread运行在SP上，Multiprocessor将Thread分为32个一组， 称之为Warp, 一个Warp每次执行一个共同的指令, 是最小的调度单元，被调度到Multiprocessor上执行。 这种机制也叫做SIMT（Single Instruction Multiple Thread）.

​	例如如下代码：

```
// Kernel definition
__global__ void MatAdd(float A[N][N], float B[N][N],
                       float C[N][N])
{
    int i = threadIdx.x;
    int j = threadIdx.y;
    C[i][j] = A[i][j] + B[i][j];
}

int main()
{
    ...
    // Kernel invocation with one block of N * N * 1 threads
    int numBlocks = 1;
    dim3 threadsPerBlock(N, N);
    MatAdd<<<numBlocks, threadsPerBlock>>>(A, B, C);
    ...
}
```

Kernel 为MatAdd， 执行在一个线程数是N * N * 1的block上。其中numBlocks也可以是一个dim3的变量，表示单个grid的block的分布信息。

### 内存模型

​	内存地址模型如下：

![Memory Hierarchy.](https://docs.nvidia.com/cuda/cuda-c-programming-guide/graphics/memory-hierarchy.png)

所有的 Block Thread 共享 Global 内存。一个 Block 内的 Thread 共享 Shared 内存。一个 Thread 内有寄存器，本地内存以及缓存。另外，还有2种所有现成可访问的只读内存： 常量和纹理内存空间。 对于常量内存有以下2个特性：

- 对常量内存的单次读操作可以广播到其他的“邻近(nearby)”线程，这将节约15次读取操作；
- 高速缓存。常量内存的数据将缓存起来，因此对于相同地址的连续操作将不会产生额外的内存通信量；

对于纹理内存，之所以称之为 “纹理”，是因为最初是为图形应用设计的。 当程序中存在大量局部空间操作时，纹理内存可以提高性能：

*  纹理内存也是缓存在片上的，因此一些情况下相比从芯片外的DRAM上获取数据，纹理内存可以通过减少内存请求来提高带宽。
* 纹理内存是针对图形应用设计的，可以更高效地处理**局部空间的内存访问**。

### 通信模型

在Cuda中，设备与设备以及设备和主机之间的通信方式主要有同步函数、volatile关键字、ATOM操作以及VOTE操作。

#### 同步函数

1. ***__syncthreads*** 实现线程块中的线程同步，保证线程块中所有线程执行到同一位置。一个warp内的线程不用同步，只有当整个线程块都走相同分支的时候，才能在条件中使用该函数；

2. memory fence函数:  保证不同线程块之间的全局、共享数据通信的可靠性，并不要求所有函数都运行到同一位置；多个线程可以正确的操作共享数据，实现grid/block内的线程间通信:

   `__threadfence/__threadfence_block/__threadfence_system`, 调用该函数之后， 该线程在此语句前对全局存储器或共享存储器的访问已经全部完成，且结果对block内所有线程可见。

3. kernel之间通信： pinned memory

4. GPU和CPU线程同步

   主机端代码使用cudaThreadSynchronize()确定所有设备均已运行结束。

#### Volatile 关键字

​	存在于全局变量或共享存储器中的变量通过此关键字声明为敏感变量。每次对该变量的引用都会被编译成一次真实的内存读指令。

#### Atom关键字

​	当多个线程同时访问全局或共享存储器的同一位置时，保证每个线程能够实现对共享可写数据的互斥操作－－在一个操作完成之前，其他任何线程都无法访问此地址。

```
__global__ void testAtom(int* g_odata)
{
	const unsigned int tid = blockDim.x*blockIdx.x+threadIdx.x;

	//算术原子操作
	atomicAdd(&g_odata[0],10);
	atomicMax(&g_odata[0],tid);
	atomicInc((unsigned int*)&g_odata[0],17);
	atomicCAS(&g_odata[0],tid-1,tid);

	//位原子操作
	atomicAnd(&g_odata[8],2*tid+7);
	atomicOr(&g_odata[8],1<<tid);
	atomicXor(&g_odata[8],tid);
}
```

详细见：Atomic Function[^5].

#### Warp Vote操作

```
int __all_sync(unsigned mask, int predicate);
int __any_sync(unsigned mask, int predicate);
unsigned __ballot_sync(unsigned mask, int predicate);
unsigned __activemask();
```

评估指定的warp中，非退出线程在掩码中对应的predicate是否满足条件。



### PTX ISA[^7]

​	PTX属于中间代码表示。Cuda C编译成为PTX，然后再编译成目标架构相关的代码。

​	TBD...

## 椭圆曲线基本操作

一般，椭圆曲线可以用以下二元三阶方程的形式来表示：
$$
y^2 = x^3 + ax + b
$$
其中a、b为系数。



从Golang的crypto elliptic[^2] 里面提供了椭圆曲线的基本操作：

```
type CurveParams
    func (curve *CurveParams) Add(x1, y1, x2, y2 *big.Int) (*big.Int, *big.Int)
    func (curve *CurveParams) Double(x1, y1 *big.Int) (*big.Int, *big.Int)
    func (curve *CurveParams) IsOnCurve(x, y *big.Int) bool
    func (curve *CurveParams) Params() *CurveParams
    func (curve *CurveParams) ScalarBaseMult(k []byte) (*big.Int, *big.Int)
    func (curve *CurveParams) ScalarMult(Bx, By *big.Int, k []byte) (*big.Int, *big.Int)
```

​	其中涉及到点加、标量乘法，因为是素域上的运算，在运算过程中需要求逆。

​	对于非有限域上的点加， 在椭圆曲线上人去一点P, 再取一点Q, 链接P、Q作一条直线，这条线将在椭圆曲线上交于第三点G，过G做垂直于X轴的直线，将过椭圆曲线的另外一个点R。R则被定义为P + Q的结果：
$$
P = Q + R
$$
<img src="/Users/duanbing/Library/Application Support/typora-user-images/image-20200608162356167.png" alt="image-20200608162356167" style="zoom:50%;" />



当P=Q的情况下， 直线是P(Q)点的切线，同样能找到G，求出R。

实际计算的时候， 首先计算P-Q直线的斜率:
$$
\lambda =\left\{
\begin{aligned}
x & = & (y_p - y_q)/(x_p - x_q), P != Q \\
y & = & (3x_p^2 + a)/y_p, P = Q\\
\end{aligned}
\right.
$$
对于有限域(p为域的阶)， 需要离散化，求”商“要变成分数求模，也就是求逆元。
$$
\lambda =\left\{
\begin{aligned}
x & = & (y_p - y_q)/(x_p - x_q)\ mod\ p, P != Q \\
y & = & (3x_p^2 + a)/y_p\ mod\ p, P = Q\\
\end{aligned}
\right.
$$


那么求出来的最终的R的坐标为:
$$
\begin{align}
x_r &= (\lambda^2 - x_p - x_q)\ mod\ p \\
y_r &= (\lambda(x_p - x_r) - y_p)\ mod\ p
\end{align}
$$
转成Jacobian格式(转换方法： (x, y) ->  (x',y', 1),  (x', y', z) -> (x= x' /z²,y= y' /z³))之后，计算流程如下（推导过过程省略）：
$$
\begin{align}
u1 &= x1\cdot z2^2   \\
u2 &= x2\cdot z1^2   \\
s1 &= y1\cdot z2^3   \\
s2 &= y2\cdot z1^3	 \\
h &= u2 - u1    \\
r &= s2 - s1  \\
x3 &= r^2 - h^2 - 2\cdot u1\cdot h^2  \\
y3 &= r\cdot (u1\cdot h^2 - x3) - s1\cdot h^3  \\
z3 &= z1\cdot z2\cdot h  \\
\end{align}
$$
代码实现参考： [Add](https://golang.org/src/crypto/elliptic/elliptic.go?s=3250:3325#L92)

对于标量乘法k*G来说，实际上可以分解为k-1次加法。但是为了提升效率，一般蒙哥马利算法, 具体参考[这里](https://github.com/duanbing/xchain-rust-crypto/blob/master/xchain-rust-crypto-intrro.pdf)。标量乘法是RSA以及ECC中使用频率非常高的运算，点乘和倍乘都依赖于标量乘法。

​	因此后面主要分析目前基于Cuda是如何加速点乘的实现。

​	文献RSA2048 on GPGPUs on cuda[^6]、The Montgomery Powering Ladder等都是Montgomery的形式上开展的点乘计算的优化， 减少计算过程中的模操作。

​	Montgomery形式[^8][^9]:
$$
a\ mod\ N \rightarrow a' = aR\ mod\ N
$$
​	其中R > N, gcd(R, N) = 1.  

​	按照这种形式计算的结果是 (abR)R mod N, 需要再除去R， 才能变回结果的Montgomery形式:
$$
a \cdot b\ mod\ N = a'b'R^{-1}\ mod\ N
$$
这就需要非常高效的计算TR^-1的算法，利用Montgomery reduction 算法如下：

```
function REDC is
    input: Integers R and N with gcd(R, N) = 1,
           Integer N′ in [0, R − 1] such that NN′ ≡ −1 mod R,
           Integer T in the range [0, RN − 1]
    output: Integer S in the range [0, N − 1] such that S ≡ TR^−1 mod N

    m ← ((T mod R)N′) mod R
    t ← (T + mN) / R
    if t ≥ N then
        return t − N
    else
        return t
    end if
end function
```

可以看到， 在这个求TR^−1（也就是T就是点乘的结果）的计算过程中，通过预处理， 模操作只需要一次判断和一次减法。

​	另外，Montgomery Powering Ladder[^10]方法， 将g^k的计算，将计算的结果分为2部分，low half 和high half, 分别采用2个乘法器并行计算。

​	文献[^6]给出了利用cuda加速RSA的方案。利用Montgomery形式，将a和b换算为32进制（对应一个warp的32个线程），利用普通的竖式的乘法方式，在单个线程里面将被乘数不同次数(32的幂)对应的系数单独跟乘数进行计算，最后再单独处理进位。 

## 已有实现调研

### Cuda大数库cuda-fixnum[^4]

   fixnum实现了大数的算术运算和模运算。支持的大数长度包括32,64,128,256,512,1024 和 2048。核心是Montgomery算法实现的模乘。

​	安装和测试（ubuntu）：

```
  sudo apt-get install libgmp3-dev libmpc-dev -y
  python3 tests/gentests.py
  make check
```

mul_wide是模乘的基础：

```
 /*
     * (s, r) = a * b
     *
     * r is the "lo half" (see mul_lo above) and s is the
     * corresponding "hi half".
     */
    __device__ static void mul_wide(fixnum &ss, fixnum &rr, fixnum a, fixnum b) {
        int L = layout::laneIdx();  // 当前线程在warp中的id

        fixnum r, s;
        r = fixnum::zero();
        s = fixnum::zero();
        digit cy = digit::zero();

        fixnum ai = get(a, 0); // 获得a_0， 见下图 
        digit::mul_lo(s, ai, b);  // 计算ai * b, 将lo_half存放在s上，
        r = L == 0 ? s : r;  // r[0] = s[0]; 
        s = layout::shfl_down0(s, 1);  // 移动lane id 加1，用在存放高位计算的结果 
        digit::mad_hi_cy(s, cy, ai, b, s);  //将ai * b的hi_half和lo_half的进位加起来，放在上面找到的位置，下面就是处理1-32位。

        for (int i = 1; i < layout::WIDTH; ++i) {
            fixnum ai = get(a, i);  //
            digit::mad_lo_cc(s, ai, b, s);

            fixnum s0 = get(s, 0);
            r = (L == i) ? s0 : r; // r[i] = s[0]
            s = layout::shfl_down0(s, 1);

            // TODO: Investigate whether deferring this carry resolution until
            // after the loop improves performance much.
            digit::addc_cc(s, s, cy);  // add carry from prev digit
            digit::addc(cy, 0, 0);     // cy = CC.CF
            digit::mad_hi_cy(s, cy, ai, b, s);
        }
        cy = layout::shfl_up0(cy, 1);
        add(s, s, cy);
        rr = r;
        ss = s;
    }

```

同时这个计算过程也可以通过文档[^6]看到详细的计算过程。

​	指数模运算就是在mul_wide的基础之上实现的Montgomery算法以及CIOS[^12]优化的实现。 

​	最后在应用方面，项目[^11]利用了cuda-fixnum进行groth16的加速。

#### cuda-fixnum性能测试

配置： 

```
使用GPU device 0: Tesla T4
设备全局内存总量： 15079MB
SM的数量：40
每个线程块的共享内存大小：48 KB
每个线程块的最大线程数：1024
设备上一个线程块（Block）种可用的32位寄存器数量： 65536
每个EM的最大线程数：1024
每个EM的最大线程束数：32
设备上多处理器的数量： 40
```

具体性能测试数据如下：

```
Function: mul_wide, #elts: 5000e3
fixnum digit  total data   time       Kops/s
 bits  bits     (MiB)    (seconds)
   32    32      19.1     0.000    26595744.7
   64    32      38.1     0.000    13850415.5
  128    32      76.3     0.001     7183908.0
  256    32     152.6     0.001     3591954.0
  512    32     305.2     0.006      898149.8
 1024    32     610.4     0.015      322705.6

   64    64      38.1     0.000    13927576.6
  128    64      76.3     0.001     7267441.9
  256    64     152.6     0.001     3709198.8
  512    64     305.2     0.005      950028.5
 1024    64     610.4     0.011      467158.7
 2048    64    1220.7     0.038      132848.0
```

可以看到，对于普通的256位点乘，可以达到 3,709,198,000 ops/s.  相对于golang原始的大数乘法快一倍。																

## 参考

[^1]: https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#introduction 
[^2]: https://golang.org/pkg/crypto/elliptic
[^3]: https://developer.nvidia.com/cuda-gpus
[^4]: https://github.com/unzvfu/cuda-fixnum
[^5]: https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#atomic-functions
[^6 ]: https://www.marcelokaihara.com/papers/An_Implementation_of_RSA2048_on_GPUs_using_CUDA.pdf
[^7]: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#ptx-machine-model
[^8]: https://en.wikipedia.org/wiki/Montgomery_modular_multiplication
[^9]: https://cp-algorithms.com/algebra/montgomery_multiplication.html
[^10]: http://cr.yp.to/bib/2003/joye-ladder.pdf
[^11]: https://github.com/CodaProtocol/gpu-groth16-prover-3x

[^12]: http://www.just.edu.jo/~tawalbeh/nyit/csci860/notes/high.pdf

