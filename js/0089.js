/**
 * @param {number} n
 * @return {number[]}
 */
var grayCode = function(n) {
  var ans=new Array()
  for(var i=0;i<(1<<n);i++){
    ans.push(i^(i>>1))
  }
  return ans
};