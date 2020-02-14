# Joint Attack


General Torstein has sent the x-coordinate for the next joint attack and is expecting you to promptly follow his orders in order to avoid impeding doom. Unfortunately Torstein hates numbers with more than 2 digits and loves horizontal line segments, and has therefore sent the coordinate as a continued fraction, i.e.

![alt text][img1]

Your rocket launcher only accepts coordinates as reduced fractions, so you need to quickly compute the correct numbers to feed it in order to commence the attack. Hurry! Failure may have dire consequences!

## Input

The first line of output is one integer n (1 <= n < 40), the number of coefficients in the continued fraction, followed by a line with n integers (1 <= x_ i < 100) the coefficients of x.

## Output

The coordinate x as a reduced fraction. It is guaranteed that the numerator and denominator are both less than 10<sup>18</sup>.

<table class="sample" summary="sample data">

<tbody>

<tr>

<th>Sample Input 1</th>

<th>Sample Output 1</th>

</tr>

<tr>

<td>

<pre>2
2 3
</pre>

</td>

<td>

<pre>7/3
</pre>

</td>

</tr>

</tbody>

</table>



[img1]: .\img1.png
