// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

address constant SWAP_ROUTER_02 = 0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45;
address constant UNISWAP_V2_ROUTER = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

contract UniswapV3FlashSwap {
    ISwapRouter02 constant router = ISwapRouter02(SWAP_ROUTER_02);
    IUniswapV2Router02 constant v2_router = IUniswapV2Router02(UNISWAP_V2_ROUTER);
    uint160 private constant MIN_SQRT_RATIO = 4295128739;
    uint160 private constant MAX_SQRT_RATIO =
        1461446703485210103287273052203988822378723970342;

    function flashSwap_V3_to_V2(
        address pool0,
        uint24 fee1,
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) external {
        bool zeroForOne = tokenIn < tokenOut;
        // 0 -> 1 => sqrt price decrease
        // 1 -> 0 => sqrt price increase
        uint160 sqrtPriceLimitX96 =
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1;

        bytes memory data = abi.encode(
            msg.sender, pool0, fee1, tokenIn, tokenOut, amountIn, zeroForOne
        );

        IUniswapV3Pool(pool0).swap({
            recipient: address(this),
            zeroForOne: zeroForOne,
            amountSpecified: int256(amountIn),
            sqrtPriceLimitX96: sqrtPriceLimitX96,
            data: data
        });
    }

    function _swap_v2(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOutMin
    ) private returns (uint256 amountOut)  {
        IERC20(tokenIn).approve(address(v2_router), amountIn);

        address[] memory path;
        path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;

        uint256[] memory amounts = v2_router.swapExactTokensForTokens(
            amountIn, 1, path, address(this), block.timestamp
        );
    return amounts[1];
    }



    function uniswapV3SwapCallback(
        int256 amount0,
        int256 amount1,
        bytes calldata data
    ) external {
        (
            address caller,
            address pool0,
            uint24 _fee1,
            address tokenIn,
            address tokenOut,
            uint256 amountIn,
            bool zeroForOne
        ) = abi.decode(
            data, (address, address, uint24, address, address, uint256, bool)
        );
        require(msg.sender == address(pool0), "not sender");

        uint256 amountOut = zeroForOne ? uint256(-amount1) : uint256(-amount0);
        // pool0 -> tokenIn -> tokenOut (amountOut)
        // Swap on pool 1 (swap tokenOut -> tokenIn)
        uint256 revenue = _swap_v2({
            tokenIn: tokenOut,
            tokenOut: tokenIn,
            amountIn: amountOut,
            amountOutMin: amountIn
        });

        uint256 profit = revenue - amountIn;

        require(profit > 0, "profit = 0");
        IERC20 loanedFrom = IERC20(tokenIn);
        uint256 daiBalance = loanedFrom.balanceOf(address(this));

        IERC20(tokenIn).transfer(pool0, amountIn);
        IERC20(tokenIn).transfer(address(caller), profit);
    }

    function flashSwap_V2_to_V3(
         address pool0,
         uint24 fee1,
         address tokenIn,
         address tokenOut,
         uint256 amountIn
     ) external {

         bool zeroForOne = tokenIn < tokenOut;
         // 0 -> 1 => sqrt price decrease
         // 1 -> 0 => sqrt price increase
         uint160 sqrtPriceLimitX96 =
             zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1;


        address factory = 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f;

        address pair = IUniswapV2Factory(factory).getPair(tokenIn, tokenOut);

        IUniswapV2Pair pairContract = IUniswapV2Pair(pair);

        (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast) = pairContract.getReserves();
        uint amountOut = v2_router.quote(amountIn, reserve0, reserve1);
        require(amountIn <= reserve0, "Reserves Too Low");
        require(amountOut <= reserve1, "Reserves Too Low");

        bytes memory data = abi.encode(
                  msg.sender, pair, fee1, tokenIn, tokenOut, amountIn, amountOut, zeroForOne
              );

         IUniswapV2Pool(pool0).swap(amountIn, amountOut, address(this), data);
     }

    function uniswapV2Call(
           address sender,
           uint256 amount0,
           uint256 amount1,
           bytes calldata data
       ) external {
           (address caller, address pair, uint24 fee1, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, bool zeroForOne) = abi.decode(data, (address,address, uint24, address, address, uint256, uint256, bool));

           uint256 revenue = _swap({
               tokenIn: tokenOut, // previous tokenIn
               tokenOut: tokenIn, // previous tokenOut
               fee: fee1,
               amountIn: amountOut,
               amountOutMin: amountIn
           });
           IERC20 loanedFrom = IERC20(tokenIn);
           // about 0.3% fee, +1 to round up
           uint256 amountToRepay = amount0 + (amount0 * 3 ) / 997 + (1 * 1e18);

           uint256 profit = revenue - amountIn;

           require(revenue > amountToRepay, "No Profit");

           uint256 daiBalance = loanedFrom.balanceOf(address(this));
           loanedFrom.transfer(address(pair), amountToRepay);
           loanedFrom.transfer(caller, profit);
       }
    function _swap(
            address tokenIn,
            address tokenOut,
            uint24 fee,
            uint256 amountIn,
            uint256 amountOutMin
        ) private returns (uint256 amountOut) {
            IERC20(tokenIn).approve(address(router), amountIn);

            ISwapRouter02.ExactInputSingleParams memory params = ISwapRouter02
                .ExactInputSingleParams({
                tokenIn: tokenIn,
                tokenOut: tokenOut,
                fee: fee,
                recipient: address(this),
                amountIn: amountIn,
                amountOutMinimum: amountOutMin,
                sqrtPriceLimitX96: 0
            });
            amountOut = router.exactInputSingle(params);
            return amountOut;
        }
}

interface ISwapRouter02 {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    function exactInputSingle(ExactInputSingleParams calldata params)
        external
        payable
        returns (uint256 amountOut);
}

interface IUniswapV2Router02 {
    function swapExactTokensForTokens(
        uint amountIn,
        uint amountOutMin,
        address[] calldata path,
        address to,
        uint deadline
    ) external returns (uint[] memory amounts);

    function swapTokensForExactTokens(
        uint256 amountOut,
        uint256 amountInMax,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external returns (uint256[] memory amounts);
    function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts);
    function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts);
    function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline)
           external
           payable
           returns (uint[] memory amounts);
    function quote(uint amountA, uint reserveA, uint reserveB) external pure returns (uint amountB);

}

interface IUniswapV3Pool {
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);
}

interface IUniswapV2Pool {
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
}

interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount)
        external
        returns (bool);
    function allowance(address owner, address spender)
        external
        view
        returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount)
        external
        returns (bool);
}

interface IWETH is IERC20 {
    function deposit() external payable;
    function withdraw(uint256 amount) external;
}

interface IUniswapV2Pair {
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
}

interface IUniswapV2Factory {
    function getPair(address tokenA, address tokenB) external view returns (address pair);
}
