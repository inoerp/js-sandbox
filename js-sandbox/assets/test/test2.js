 function thirdFunc(){
  Deno.core.print("\n Hello runjs!");
    console.log('\n Hello, World from thirdFunc');
    return 10;
  }

Deno.core.print("\n Hello runjs outside!");


  function fourthFunc(){
    console.log('\n Hello, World from fourthFunc!');
  }

  export {
    thirdFunc,
    fourthFunc
  }

