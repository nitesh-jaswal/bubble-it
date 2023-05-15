 let $=el=>{
    return document.querySelectorAll(el);
}

let user=[
          "king knight",
          "Ghost",
          "Ab",
          "Sycophants",          
          "Nexus",
          "Nxt",
          "Bots"          
];
let link=[
          "https://i.ibb.co/xgXpYtV/stargazing-9.jpg" ,
          "https://i.ibb.co/k9GP9rK/a93258c1cef4b582106f6ff6fa5a7bf6.jpg" , 
          "https://i.ibb.co/wC88CtN/stargazing-1.jpg" ,
          "https://i.ibb.co/CtN0kpW/Ghost-in-the-Shell-SAC-2045-2020.png" ,
          "https://i.ibb.co/VJ0W6z3/wp7267030-solo-leveling-hd-mobile-wallpapers.png" , 
          "https://i.ibb.co/cCrvHFJ/stargazing-8.jpg" ,
          "https://i.ibb.co/S7S7kDB/stargazing-7.jpg" ,
                            
];
let info=[
         [" what's up Coding Master","6:30",""],
         ["i am ghost","2:40",""],
         ["Master Coder ","13:45",""],
         [" I am nexus the future","9:20",""],
         ["NXT plan is lunched ","11:11",""],         
         ["luncheing in 30 min ","11:20",""],
         ["testing....","10:07",""]
          
];
let message;
let tab=[".t1", ".t2", ".t3",  ];
onload=()=>{
    

    $(".n1 .list")[0].onmouseover=()=>{
        $(".n1 .list")[0].style.clipPath="circle(100%)";
        $(".n1 .list")[0].style.opacity="2";
    }
    $(".n1 .list")[0].onmouseleave=()=>{
        $(".n1 .list")[0].style.clipPath="circle(30% at 100% 0%)";
        $(".n1 .list")[0].style.opacity="0";
    }
    
    for(let i in tab){
        $(tab[i])[0].onclick=()=>{
            $(".dash")[0].style.transform=`translateX(${i*100}%)`;
        }
    }
    
    $(".fa-search")[0].onclick=()=>{
        $(".searchbox")[0].style.clipPath="circle(100% at 50% 50%)";
        $(".n2")[0].style.transform="translateY(-101%)";
        setTimeout(()=>{
        $(".n2")[0].style.position="absolute";
        },200)
    }
    $(".close")[0].onclick=()=>{
        $(".searchbox")[0].style.clipPath="circle(0% at 50% 50%)";
        $(".n2")[0].style.transform="translateY(0%)";
        $(".n2")[0].style.position="";
    }
    
    for(let i=0;i<user.length;i++){
        message=`<div class="chat">
                    <span class="avatar">
                        <img src="${link[i]}">
                    </span>
                    
                    <span class="container ${user[i]}">
                        <span class="info">
                            <span>${user[i]}</span>
                            <span>${info[i][1]}</span>
                        </span>
                        
                        <span class="message">
                        <span>${info[i][0]}</span>
                        <span>${info[i][2]}</span>
                        </span>
                    </span>
                </div>`;
                
            $("section")[0].innerHTML+=message;
    }
    $(".container")[user.length-1].style.border="none";
    $(".message")[user.length-1].style=`
    color:var(--colour);
    font-weight:border;
    `;
    
    setTimeout(()=>{
    main.style.display="block";
    loader.style.display="none";
    },0)    
}
