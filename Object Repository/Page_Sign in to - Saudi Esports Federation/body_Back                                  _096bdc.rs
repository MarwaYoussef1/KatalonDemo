<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Back                                  _096bdc</name>
   <tag></tag>
   <elementGuidId>99bced93-46ed-438a-a7c2-715e6cbfb3be</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>body</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>93a73af2-5347-4436-855e-84dc4ede604c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onload</name>
      <type>Main</type>
      <value>getData()</value>
      <webElementGuid>90443d0e-d1eb-423d-aadc-e58ea17eb4f0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


    
        
            
                
                                    
                
                 Back
            
            
                
            
                    
                            
                                عربي
                            
                        
                
        
    
    
        
            
                    
            
            © 2023 Saudi Esports Federation | All rights reserved.
        
        
            
                
                
      

        function InvalidMsg(textbox) {
          if (textbox.value === '') {
            textbox.setCustomValidity('Required email address');
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity('please enter a valid email address');
          } else {
            textbox.setCustomValidity('');
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get('redirect_uri');
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get('email');
            if (email) {
              document.getElementById('username').value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get('ui_locales')|| searchParams.get('kc_locale') ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + '/user/club-sign'+'?lang='+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + '&amp;pageUrl=' + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + '/user/forget-password';
        }

      
        
          
              
                
                    Log In
                    Welcome to Esports
                
               
                  
                      
                        
                        By Mobile Number
                      
                      
                  
                   
                  
                    
                      
                      By Email
                    
                    
                  
                 
                
                  
                    
                    Mobile Number
                    *
                  
                    
                         +966   Afghanistanأفغانستان+93   Albaniaألبانيا+355   Algeriaالجزائر+213   Andorraأندورا+376   Anguillaأنغيلا+1264   Argentinaالأرجنتين+54   Armeniaأرمينيا+374   Arubaأروبا+297   Australiaأستراليا+61   Austriaالنمسا+43   Azerbaijanأذربيجان+994   Bahamasالباهاما+1242   Bahrainالبحرين+973   Bangladeshبنغلاديش+880   Barbadosبربادوس+1246   Belarusروسيا البيضاء+375   Belgiumبلجيكا+32   Belizeبليز+501   Beninبنين+229   Bermudaبرمودا+1441   Bhutanبوتان+975   Boliviaبوليفيا+591   Bosnia and Herzegovinaالبوسنة والهرسك+387   Botswanaبوتسوانا+267   Brazilالبرازيل+55   Brunei Darussalamبروناي دار السلام+673   Bulgariaبلغاريا+359   Burkina Fasoبوركينا فاسو+226   Burundiبوروندي+257   Cambodiaكمبوديا+855   Cameroonالكاميرون+237   Canadaكندا+1   Cape Verdeالرأس الأخضر+238   Central African Republicجمهورية افريقيا الوسطى+236   Chadتشاد+235   Chileتشيلي+56   Chinaالصين+86   Colombiaكولومبيا+57   Comorosجزر القمر+269   Congoالكونغو+242   Costa Ricaكوستا ريكا+506   Côte d'Ivoireكوت ديفوار+225   Croatia (Hrvatska)كرواتيا (هرفاتسكا)+385   Cubaكوبا+53   Cyprusقبرص+357   Czech Republicجمهورية التشيك+420   Denmarkالدنمارك+45   Djiboutiجيبوتي+253   Dominicaدومينيكا+1767   Ecuadorالإكوادور+593   Egyptمصر+20   El Salvadorالسلفادور+503   Equatorial Guineaغينيا الإستوائية+240   Eritreaإريتريا+291   Estoniaاستونيا+372   Ethiopiaأثيوبيا+251   Fijiفيجي+679   Finlandفنلندا+358   Franceفرنسا+33   Gabonالغابون+241   Gambiaغامبيا+220   Georgiaجورجيا+995   Germanyألمانيا+49   Ghanaغانا+233   Greeceاليونان+30   Grenadaغرينادا+1473   Guatemalaغواتيمالا+502   Guineaغينيا+224   Guinea-Bissauغينيا بيساو+245   Guyanaغيانا+592   Haitiهايتي+509   Hondurasهندوراس+504   Hungaryاليونان+36   Icelandأيسلندا+354   Indiaالهند+91   Indonesiaأندونيسيا+62   Iranإيران+98   Iraqالعراق+964   Irelandأيرلندا+353   Italyإيطاليا+39   Jamaicaجامايكا+1876   Japanاليابان+81   Jordanالأردن+962   Kazakhstanكازاخستان+77   Kenyaكينيا+254   Korea (North)كوريا الشمالية+850   Korea (South)كوريا، الجنوبيه+82   Kuwaitالكويت+965   Kyrgyzstanقرغيزستان+996   Laosلاوس+856   Latviaلاتفيا+371   Lebanonلبنان+961   Lesothoليسوتو+266   Liberiaليبيريا+231   Libyaليبيا+218   Liechtensteinليختنشتاين+423   Lithuaniaليتوانيا+370   Luxembourgلوكسمبورغ+352   Macedoniaمقدونيا+389   Madagascarمدغشقر+261   Malawiمالاوي+265   Malaysiaماليزيا+60   Maldivesجزر المالديف+960   Maliمالي+223   Maltaمالطا+356   Marshall Islandsجزر مارشال+692   Mauritaniaموريتانيا+222   Mauritiusموريشيوس+230   Mexicoالمكسيك+52   Micronesiaميكرونيزيا+691   Moldovaمولدوفا+373   Monacoموناكو+377   Mongoliaمنغوليا+976   Moroccoالمغرب+212   Mozambiqueموزمبيق+258   Myanmarميانمار+95   Namibiaناميبيا+264   Nauruناورو+674   Nepalنيبال+977   Netherlandsهولندا+31   New Zealand (Aotearoa)نيوزيلندا (اوتياروا)+64   Nicaraguaنيكاراغوا+505   Nigerالنيجر+227   Norwayالنرويج+47   Omanسلطنة عمان+968   Pakistanباكستان+92   Palauبالاو+680   Palestinian Territoryالأراضي الفلسطينية+970   Panamaبناما+507   Papua New Guineaبابوا غينيا الجديدة+675   Paraguayباراغواي+595   Peruبيرو+51   Philippinesفيلبيني+63   Polandبولندا+48   Portugalالبرتغال+351   Qatarدولة قطر+974   Romaniaرومانيا+40   Russian Federationالاتحاد الروسي+7   Rwandaرواندا+250   Saint Luciaالقديسة لوسيا+1758   Samoaساموا+685   San Marinoسان مارينو+378   Sao Tome and Principeساو تومي وبرنسيبي+239   Saudi Arabiaالمملكة العربية السعودية+966   Senegalالسنغال+221   Serbiaصربيا+381   Seychellesسيشيل+248   Sierra Leoneسيرا ليون+232   Singaporeسنغافورة+65   Slovakiaسلوفاكيا+421   Sloveniaسلوفينيا+386   Solomon Islandsجزر سليمان+677   Somaliaالصومال+252   South Africaجنوب أفريقيا+27   Spainإسبانيا+34   Sri Lankaسيريلانكا+94   Sudanسودان+249   Surinameسورينام+597   Swazilandسوازيلاند+268   Swedenالسويد+46   Switzerlandسويسرا+41   Syriaسوريا+963   Taiwanتايوان+886   Tajikistanطاجيكستان+992   Tanzaniaتنزانيا+255   Thailandتايلاند+66   Timor-Lesteتيمور ليشتي+670   Togoليذهب+228   Tongaتونغا+676   Trinidad and Tobagoترينداد وتوباغو+1868   Tunisiaتونس+216   Turkeyديك رومي+90   Turkmenistanتركمانستان+993   Tuvaluتوفالو+688   Ugandaأوغندا+256   Ukraineأوكرانيا+380   United Arab Emiratesالإمارات العربية المتحدة+971   United Kingdomالمملكة المتحدة+44   United States of Americaالولايات المتحدة+1684   Uruguayأوروغواي+598   Uzbekistanأوزبكستان+998   Vanuatuفانواتي+678   Venezuelaفنزويلا+58   Viet Namفيتنام+84   Yemenاليمن+967   Zambiaزامبيا+260   Zimbabweزيمبابوي+263
                      
                         Invalid mobile number
                  
                

                 

                
                  
                    
                      
                          Email Address
                    *
                  
                      
                         please enter a vaild email
                

                
                  
                    Password*
                  
                  
                
                
                  
                
                
                

                
                    
                    
                    
                    
                          Forgot Password?
                    
                    

                
              

    
       Don’t have account yet? 
      
        Create New Account
       
    
    


    



                


      

        function InvalidMsg(textbox) {
          if (textbox.value === '') {
            textbox.setCustomValidity('Required email address');
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity('please enter a valid email address');
          } else {
            textbox.setCustomValidity('');
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get('redirect_uri');
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get('email');
            if (email) {
              document.getElementById('username').value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get('ui_locales')|| searchParams.get('kc_locale') ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + '/user/club-sign'+'?lang='+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + '&amp;pageUrl=' + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + '/user/forget-password';
        }

      





                
            
        
    




    const setLocale = () => {
        ''
            localStorage.setItem('lang', 'en');
        ''
    }

    var selectedMobileCode = '+966';
    var selectedMobilePattern= new RegExp(&quot;^(05|5)(5|0|3|6|4|9|1|8|7)([0-9]{7})$&quot;);
    function getData() {

      const loginMethod = localStorage.getItem('login_method') || 'mobile';
        document.getElementById(&quot;byMail&quot;).checked = (loginMethod === 'email');
        document.getElementById(&quot;byNumber&quot;).checked = (loginMethod !== 'email') ;
        ShowHideDiv();
      fetch('/api/team-club/v1/public/geography/country')
        .then(response => response.json())
        .then(data => {
        const listContainer = document.getElementById(&quot;dropdown&quot;);
        const dropdownbtn = document.createElement(&quot;div&quot;);
        dropdownbtn.className =&quot;dropbtn arrow&quot;;
        dropdownbtn.setAttribute('id', &quot;defult-country-flag&quot;)

        const searchplaceholder  = 'Search..';

        dropdownbtn.innerHTML = &quot;&lt;img src='/resource/esports/public/sa.png' width='25' height='25' class='country-flag'/> &amp;nbsp +966&quot;;
        listContainer.appendChild(dropdownbtn);

        const listdiv = document.createElement(&quot;div&quot;);
        listdiv.className = &quot;dropdown-content&quot;
        listdiv.id =&quot;myDropdown&quot;
        listContainer.appendChild(listdiv);

         const searchInput = document.createElement(&quot;input&quot;);
         searchInput.setAttribute('type', &quot;text&quot;);
         searchInput.setAttribute('id', &quot;myInput&quot;)
         searchInput.setAttribute('placeholder', searchplaceholder)
         searchInput.setAttribute('class', &quot;search-field&quot;)
         listdiv.appendChild(searchInput);
         dropdownbtn.addEventListener(&quot;click&quot;, () => myFunction());

         searchInput.addEventListener(&quot;keyup&quot;, () => filterFunction());

          data.forEach((item,index) => {
            const listItem = document.createElement(&quot;a&quot;);
            listItem.setAttribute('href', &quot;#&quot;);
            listItem.setAttribute('id', &quot;country-flag-&quot; + index)
            listItem.setAttribute('onclick', &quot;return false ;&quot;);
            listItem.onclick = function(){return false};
            listdiv.appendChild(listItem);

            const countryCode = document.createElement(&quot;span&quot;);
            countryCode.innerText = item.dialCode;
            countryCode.className =&quot;country-code&quot;;

            document.getElementById(&quot;country-flag-&quot; + index).innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ '/resource/esports/public/' + item.code + '.png'+&quot; width='25' height='25' class='country-flag'/> &amp;nbsp &lt;span style='display:none;'>&quot; +item.name + item.nameAr +&quot;&lt;/span>&quot;;
            listItem.appendChild(countryCode);
            listItem.addEventListener(&quot;click&quot;, () => handleItemClick(item.code, item.flag , item.dialCode,item.regex));
          });
        })
        .catch(error => console.error(error));
    }

    function myFunction() {
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
    }

    function handleItemClick(countryCode , flag , mbileCode,regex){
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
      const dropdownbtn= document.getElementById(&quot;defult-country-flag&quot;);
      selectedMobileCode = mbileCode;
      selectedMobilePattern = new RegExp(regex);
      let mobileInput = document.getElementById('mobile');
      var byMobile = document.getElementById(&quot;byNumber&quot;);
      let usernameinputField = document.getElementById('usernameinput');
      if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
            usernameinputField.value = userNameValue;
        }
      dropdownbtn.innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ '/resource/esports/public/' + countryCode + '.png'+&quot; width='25' height='25' class='country-flag'/> &amp;nbsp&quot;+ mbileCode;
    }

    function filterFunction() {
      var input, filter, ul, li, a, i;
      input = document.getElementById(&quot;myInput&quot;);
      filter = input.value.toUpperCase();
      div = document.getElementById(&quot;myDropdown&quot;);
      a = div.getElementsByTagName(&quot;a&quot;);
      for (i = 0; i &lt; a.length; i++) {
        txtValue = a[i].textContent || a[i].innerText;
        if (txtValue.toUpperCase().indexOf(filter) > -1) {
          a[i].style.display = &quot;&quot;;
        } else {
          a[i].style.display = &quot;none&quot;;
        }
      }
    }


    let emailInvalid = true;

        function checkMobilePattern() {
            const elem = document.getElementById(&quot;mobile&quot;);
            const regex = selectedMobilePattern;
            const inputControl = elem.parentElement;
            const errorDisplay = inputControl.querySelector('.error-msg-email');
            if (!regex.test(elem.value)) {
                errorDisplay.classList.remove('d-none');
                emailInvalid = true;
            } else {
                errorDisplay.classList.add('d-none');
                emailInvalid = false;
            }
            disableLoginButton();
        }

    function checkEmailPattern() {
        const elem = document.getElementById(&quot;email&quot;);
        const regex = /^(([^&lt;>()[\]\.,;:\s@\&quot;]+(\.[^&lt;>()[\]\.,;:\s@\&quot;]+)*)|(\&quot;.+\&quot;))@(([^&lt;>()[\]\.,;:\s@\&quot;]+\.)+[^&lt;>()[\]\.,;:\s@\&quot;]{2,})$/i;
        const inputControl = elem.parentElement;
        const errorDisplay = inputControl.querySelector('.error-msg-email');
        if (!regex.test(elem.value)) {
            errorDisplay.classList.remove('d-none');
            emailInvalid = true;
        } else {
            errorDisplay.classList.add('d-none');
            emailInvalid = false;
        }
        disableLoginButton();
    }

    // Disable login button if has not value
    function disableLoginButton() {
        let usernameInput = document.getElementById('email');
        let mobileInput = document.getElementById('mobile');
        let usernameinputField = document.getElementById('usernameinput');
        let passwordInput = document.getElementById('password');
        let loginButton = document.getElementById('kc-login');
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);
        let userNameValue ;
        if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
        }
        if(byMail.checked ){
            userNameValue = usernameInput.value;
        }
         usernameinputField.value = userNameValue;

        if(userNameValue &amp;&amp; passwordInput) {
            if (!userNameValue || userNameValue === '' || passwordInput.value === '' || emailInvalid) {
                loginButton.disabled = true;
            } else {
                loginButton.disabled = false;
            }
        }
    }

    function backToHome() {
        window.location.href = window.location.origin;
    }
    window.addEventListener('load', function() {
        setLocale();

        let emailInput = document.getElementById('email');
        if (emailInput &amp;&amp; emailInput.value.trim()) {
            checkEmailPattern();
        }

        disableLoginButton();

        let url = window.location.href;
        if(url.includes('execution')){
            const warningElm = document.querySelector('.pf-c-alert.pf-m-warning');
            if (warningElm) {
                document.querySelector('.pf-c-alert.pf-m-warning').style.display = 'none'; 
            }
        }
    })

     function ShowHideDiv() {
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);

        var mobileDiv = document.getElementById(&quot;mobilegroup&quot;);
        mobileDiv.style.display = byMobile.checked ? &quot;block&quot; : &quot;none&quot;;

        var emailDiv = document.getElementById(&quot;emailgroup&quot;);
        emailDiv.style.display = byMail.checked ? &quot;block&quot; : &quot;none&quot;;

        var mobilLabel = document.getElementById(&quot;numberLabel&quot;);
        var mailLabel = document.getElementById(&quot;mailLabel&quot;);

        mobilLabel.style.color = byMobile.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        mailLabel.style.color = byMail.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        localStorage.setItem('login_method', (byMail.checked ? 'email': 'mobile') )

        let usernameInput = document.getElementById('email');
        usernameInput.value = '';
        let mobileInput = document.getElementById('mobile');
        mobileInput.value = '';
        let usernameinputField = document.getElementById('usernameinput');
        usernameinputField.value = '';

        var errmsg = document.getElementById(&quot;errmsg&quot;);
        if(errmsg){
            if(byMail.checked){
                errmsg.innerHTML = 'The email or password is incorrect, Please double-check and try again.';
            }
            if(byMobile.checked){
                errmsg.innerHTML = 'The mobile number or password is incorrect, Please double-check and try again.';
            }
        }
     }



id(&quot;mobile&quot;)</value>
      <webElementGuid>033ef88e-c40d-4083-b363-eeed105975f8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;login-pf English&quot;]/body[1]</value>
      <webElementGuid>00d9db3a-c9be-40c3-b17a-f3f4a1cbe1b7</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>642d7b8a-155e-4351-b502-76b417f918e8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;


    
        
            
                
                                    
                
                 Back
            
            
                
            
                    
                            
                                عربي
                            
                        
                
        
    
    
        
            
                    
            
            © 2023 Saudi Esports Federation | All rights reserved.
        
        
            
                
                
      

        function InvalidMsg(textbox) {
          if (textbox.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;Required email address&quot; , &quot;'&quot; , &quot;);
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;please enter a valid email address&quot; , &quot;'&quot; , &quot;);
          } else {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get(&quot; , &quot;'&quot; , &quot;redirect_uri&quot; , &quot;'&quot; , &quot;);
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
            if (email) {
              document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;).value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get(&quot; , &quot;'&quot; , &quot;ui_locales&quot; , &quot;'&quot; , &quot;)|| searchParams.get(&quot; , &quot;'&quot; , &quot;kc_locale&quot; , &quot;'&quot; , &quot;) ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + &quot; , &quot;'&quot; , &quot;/user/club-sign&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;?lang=&quot; , &quot;'&quot; , &quot;+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + &quot; , &quot;'&quot; , &quot;&amp;pageUrl=&quot; , &quot;'&quot; , &quot; + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + &quot; , &quot;'&quot; , &quot;/user/forget-password&quot; , &quot;'&quot; , &quot;;
        }

      
        
          
              
                
                    Log In
                    Welcome to Esports
                
               
                  
                      
                        
                        By Mobile Number
                      
                      
                  
                   
                  
                    
                      
                      By Email
                    
                    
                  
                 
                
                  
                    
                    Mobile Number
                    *
                  
                    
                         +966   Afghanistanأفغانستان+93   Albaniaألبانيا+355   Algeriaالجزائر+213   Andorraأندورا+376   Anguillaأنغيلا+1264   Argentinaالأرجنتين+54   Armeniaأرمينيا+374   Arubaأروبا+297   Australiaأستراليا+61   Austriaالنمسا+43   Azerbaijanأذربيجان+994   Bahamasالباهاما+1242   Bahrainالبحرين+973   Bangladeshبنغلاديش+880   Barbadosبربادوس+1246   Belarusروسيا البيضاء+375   Belgiumبلجيكا+32   Belizeبليز+501   Beninبنين+229   Bermudaبرمودا+1441   Bhutanبوتان+975   Boliviaبوليفيا+591   Bosnia and Herzegovinaالبوسنة والهرسك+387   Botswanaبوتسوانا+267   Brazilالبرازيل+55   Brunei Darussalamبروناي دار السلام+673   Bulgariaبلغاريا+359   Burkina Fasoبوركينا فاسو+226   Burundiبوروندي+257   Cambodiaكمبوديا+855   Cameroonالكاميرون+237   Canadaكندا+1   Cape Verdeالرأس الأخضر+238   Central African Republicجمهورية افريقيا الوسطى+236   Chadتشاد+235   Chileتشيلي+56   Chinaالصين+86   Colombiaكولومبيا+57   Comorosجزر القمر+269   Congoالكونغو+242   Costa Ricaكوستا ريكا+506   Côte d&quot; , &quot;'&quot; , &quot;Ivoireكوت ديفوار+225   Croatia (Hrvatska)كرواتيا (هرفاتسكا)+385   Cubaكوبا+53   Cyprusقبرص+357   Czech Republicجمهورية التشيك+420   Denmarkالدنمارك+45   Djiboutiجيبوتي+253   Dominicaدومينيكا+1767   Ecuadorالإكوادور+593   Egyptمصر+20   El Salvadorالسلفادور+503   Equatorial Guineaغينيا الإستوائية+240   Eritreaإريتريا+291   Estoniaاستونيا+372   Ethiopiaأثيوبيا+251   Fijiفيجي+679   Finlandفنلندا+358   Franceفرنسا+33   Gabonالغابون+241   Gambiaغامبيا+220   Georgiaجورجيا+995   Germanyألمانيا+49   Ghanaغانا+233   Greeceاليونان+30   Grenadaغرينادا+1473   Guatemalaغواتيمالا+502   Guineaغينيا+224   Guinea-Bissauغينيا بيساو+245   Guyanaغيانا+592   Haitiهايتي+509   Hondurasهندوراس+504   Hungaryاليونان+36   Icelandأيسلندا+354   Indiaالهند+91   Indonesiaأندونيسيا+62   Iranإيران+98   Iraqالعراق+964   Irelandأيرلندا+353   Italyإيطاليا+39   Jamaicaجامايكا+1876   Japanاليابان+81   Jordanالأردن+962   Kazakhstanكازاخستان+77   Kenyaكينيا+254   Korea (North)كوريا الشمالية+850   Korea (South)كوريا، الجنوبيه+82   Kuwaitالكويت+965   Kyrgyzstanقرغيزستان+996   Laosلاوس+856   Latviaلاتفيا+371   Lebanonلبنان+961   Lesothoليسوتو+266   Liberiaليبيريا+231   Libyaليبيا+218   Liechtensteinليختنشتاين+423   Lithuaniaليتوانيا+370   Luxembourgلوكسمبورغ+352   Macedoniaمقدونيا+389   Madagascarمدغشقر+261   Malawiمالاوي+265   Malaysiaماليزيا+60   Maldivesجزر المالديف+960   Maliمالي+223   Maltaمالطا+356   Marshall Islandsجزر مارشال+692   Mauritaniaموريتانيا+222   Mauritiusموريشيوس+230   Mexicoالمكسيك+52   Micronesiaميكرونيزيا+691   Moldovaمولدوفا+373   Monacoموناكو+377   Mongoliaمنغوليا+976   Moroccoالمغرب+212   Mozambiqueموزمبيق+258   Myanmarميانمار+95   Namibiaناميبيا+264   Nauruناورو+674   Nepalنيبال+977   Netherlandsهولندا+31   New Zealand (Aotearoa)نيوزيلندا (اوتياروا)+64   Nicaraguaنيكاراغوا+505   Nigerالنيجر+227   Norwayالنرويج+47   Omanسلطنة عمان+968   Pakistanباكستان+92   Palauبالاو+680   Palestinian Territoryالأراضي الفلسطينية+970   Panamaبناما+507   Papua New Guineaبابوا غينيا الجديدة+675   Paraguayباراغواي+595   Peruبيرو+51   Philippinesفيلبيني+63   Polandبولندا+48   Portugalالبرتغال+351   Qatarدولة قطر+974   Romaniaرومانيا+40   Russian Federationالاتحاد الروسي+7   Rwandaرواندا+250   Saint Luciaالقديسة لوسيا+1758   Samoaساموا+685   San Marinoسان مارينو+378   Sao Tome and Principeساو تومي وبرنسيبي+239   Saudi Arabiaالمملكة العربية السعودية+966   Senegalالسنغال+221   Serbiaصربيا+381   Seychellesسيشيل+248   Sierra Leoneسيرا ليون+232   Singaporeسنغافورة+65   Slovakiaسلوفاكيا+421   Sloveniaسلوفينيا+386   Solomon Islandsجزر سليمان+677   Somaliaالصومال+252   South Africaجنوب أفريقيا+27   Spainإسبانيا+34   Sri Lankaسيريلانكا+94   Sudanسودان+249   Surinameسورينام+597   Swazilandسوازيلاند+268   Swedenالسويد+46   Switzerlandسويسرا+41   Syriaسوريا+963   Taiwanتايوان+886   Tajikistanطاجيكستان+992   Tanzaniaتنزانيا+255   Thailandتايلاند+66   Timor-Lesteتيمور ليشتي+670   Togoليذهب+228   Tongaتونغا+676   Trinidad and Tobagoترينداد وتوباغو+1868   Tunisiaتونس+216   Turkeyديك رومي+90   Turkmenistanتركمانستان+993   Tuvaluتوفالو+688   Ugandaأوغندا+256   Ukraineأوكرانيا+380   United Arab Emiratesالإمارات العربية المتحدة+971   United Kingdomالمملكة المتحدة+44   United States of Americaالولايات المتحدة+1684   Uruguayأوروغواي+598   Uzbekistanأوزبكستان+998   Vanuatuفانواتي+678   Venezuelaفنزويلا+58   Viet Namفيتنام+84   Yemenاليمن+967   Zambiaزامبيا+260   Zimbabweزيمبابوي+263
                      
                         Invalid mobile number
                  
                

                 

                
                  
                    
                      
                          Email Address
                    *
                  
                      
                         please enter a vaild email
                

                
                  
                    Password*
                  
                  
                
                
                  
                
                
                

                
                    
                    
                    
                    
                          Forgot Password?
                    
                    

                
              

    
       Don’t have account yet? 
      
        Create New Account
       
    
    


    



                


      

        function InvalidMsg(textbox) {
          if (textbox.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;Required email address&quot; , &quot;'&quot; , &quot;);
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;please enter a valid email address&quot; , &quot;'&quot; , &quot;);
          } else {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get(&quot; , &quot;'&quot; , &quot;redirect_uri&quot; , &quot;'&quot; , &quot;);
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
            if (email) {
              document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;).value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get(&quot; , &quot;'&quot; , &quot;ui_locales&quot; , &quot;'&quot; , &quot;)|| searchParams.get(&quot; , &quot;'&quot; , &quot;kc_locale&quot; , &quot;'&quot; , &quot;) ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + &quot; , &quot;'&quot; , &quot;/user/club-sign&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;?lang=&quot; , &quot;'&quot; , &quot;+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + &quot; , &quot;'&quot; , &quot;&amp;pageUrl=&quot; , &quot;'&quot; , &quot; + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + &quot; , &quot;'&quot; , &quot;/user/forget-password&quot; , &quot;'&quot; , &quot;;
        }

      





                
            
        
    




    const setLocale = () => {
        &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            localStorage.setItem(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;);
        &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    }

    var selectedMobileCode = &quot; , &quot;'&quot; , &quot;+966&quot; , &quot;'&quot; , &quot;;
    var selectedMobilePattern= new RegExp(&quot;^(05|5)(5|0|3|6|4|9|1|8|7)([0-9]{7})$&quot;);
    function getData() {

      const loginMethod = localStorage.getItem(&quot; , &quot;'&quot; , &quot;login_method&quot; , &quot;'&quot; , &quot;) || &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot;byMail&quot;).checked = (loginMethod === &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot;byNumber&quot;).checked = (loginMethod !== &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;) ;
        ShowHideDiv();
      fetch(&quot; , &quot;'&quot; , &quot;/api/team-club/v1/public/geography/country&quot; , &quot;'&quot; , &quot;)
        .then(response => response.json())
        .then(data => {
        const listContainer = document.getElementById(&quot;dropdown&quot;);
        const dropdownbtn = document.createElement(&quot;div&quot;);
        dropdownbtn.className =&quot;dropbtn arrow&quot;;
        dropdownbtn.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;defult-country-flag&quot;)

        const searchplaceholder  = &quot; , &quot;'&quot; , &quot;Search..&quot; , &quot;'&quot; , &quot;;

        dropdownbtn.innerHTML = &quot;&lt;img src=&quot; , &quot;'&quot; , &quot;/resource/esports/public/sa.png&quot; , &quot;'&quot; , &quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp +966&quot;;
        listContainer.appendChild(dropdownbtn);

        const listdiv = document.createElement(&quot;div&quot;);
        listdiv.className = &quot;dropdown-content&quot;
        listdiv.id =&quot;myDropdown&quot;
        listContainer.appendChild(listdiv);

         const searchInput = document.createElement(&quot;input&quot;);
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, &quot;text&quot;);
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;myInput&quot;)
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;, searchplaceholder)
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, &quot;search-field&quot;)
         listdiv.appendChild(searchInput);
         dropdownbtn.addEventListener(&quot;click&quot;, () => myFunction());

         searchInput.addEventListener(&quot;keyup&quot;, () => filterFunction());

          data.forEach((item,index) => {
            const listItem = document.createElement(&quot;a&quot;);
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, &quot;#&quot;);
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;country-flag-&quot; + index)
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;, &quot;return false ;&quot;);
            listItem.onclick = function(){return false};
            listdiv.appendChild(listItem);

            const countryCode = document.createElement(&quot;span&quot;);
            countryCode.innerText = item.dialCode;
            countryCode.className =&quot;country-code&quot;;

            document.getElementById(&quot;country-flag-&quot; + index).innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ &quot; , &quot;'&quot; , &quot;/resource/esports/public/&quot; , &quot;'&quot; , &quot; + item.code + &quot; , &quot;'&quot; , &quot;.png&quot; , &quot;'&quot; , &quot;+&quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp &lt;span style=&quot; , &quot;'&quot; , &quot;display:none;&quot; , &quot;'&quot; , &quot;>&quot; +item.name + item.nameAr +&quot;&lt;/span>&quot;;
            listItem.appendChild(countryCode);
            listItem.addEventListener(&quot;click&quot;, () => handleItemClick(item.code, item.flag , item.dialCode,item.regex));
          });
        })
        .catch(error => console.error(error));
    }

    function myFunction() {
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
    }

    function handleItemClick(countryCode , flag , mbileCode,regex){
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
      const dropdownbtn= document.getElementById(&quot;defult-country-flag&quot;);
      selectedMobileCode = mbileCode;
      selectedMobilePattern = new RegExp(regex);
      let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
      var byMobile = document.getElementById(&quot;byNumber&quot;);
      let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
      if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
            usernameinputField.value = userNameValue;
        }
      dropdownbtn.innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ &quot; , &quot;'&quot; , &quot;/resource/esports/public/&quot; , &quot;'&quot; , &quot; + countryCode + &quot; , &quot;'&quot; , &quot;.png&quot; , &quot;'&quot; , &quot;+&quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp&quot;+ mbileCode;
    }

    function filterFunction() {
      var input, filter, ul, li, a, i;
      input = document.getElementById(&quot;myInput&quot;);
      filter = input.value.toUpperCase();
      div = document.getElementById(&quot;myDropdown&quot;);
      a = div.getElementsByTagName(&quot;a&quot;);
      for (i = 0; i &lt; a.length; i++) {
        txtValue = a[i].textContent || a[i].innerText;
        if (txtValue.toUpperCase().indexOf(filter) > -1) {
          a[i].style.display = &quot;&quot;;
        } else {
          a[i].style.display = &quot;none&quot;;
        }
      }
    }


    let emailInvalid = true;

        function checkMobilePattern() {
            const elem = document.getElementById(&quot;mobile&quot;);
            const regex = selectedMobilePattern;
            const inputControl = elem.parentElement;
            const errorDisplay = inputControl.querySelector(&quot; , &quot;'&quot; , &quot;.error-msg-email&quot; , &quot;'&quot; , &quot;);
            if (!regex.test(elem.value)) {
                errorDisplay.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
                emailInvalid = true;
            } else {
                errorDisplay.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
                emailInvalid = false;
            }
            disableLoginButton();
        }

    function checkEmailPattern() {
        const elem = document.getElementById(&quot;email&quot;);
        const regex = /^(([^&lt;>()[\]\.,;:\s@\&quot;]+(\.[^&lt;>()[\]\.,;:\s@\&quot;]+)*)|(\&quot;.+\&quot;))@(([^&lt;>()[\]\.,;:\s@\&quot;]+\.)+[^&lt;>()[\]\.,;:\s@\&quot;]{2,})$/i;
        const inputControl = elem.parentElement;
        const errorDisplay = inputControl.querySelector(&quot; , &quot;'&quot; , &quot;.error-msg-email&quot; , &quot;'&quot; , &quot;);
        if (!regex.test(elem.value)) {
            errorDisplay.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
            emailInvalid = true;
        } else {
            errorDisplay.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
            emailInvalid = false;
        }
        disableLoginButton();
    }

    // Disable login button if has not value
    function disableLoginButton() {
        let usernameInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
        let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
        let passwordInput = document.getElementById(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
        let loginButton = document.getElementById(&quot; , &quot;'&quot; , &quot;kc-login&quot; , &quot;'&quot; , &quot;);
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);
        let userNameValue ;
        if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
        }
        if(byMail.checked ){
            userNameValue = usernameInput.value;
        }
         usernameinputField.value = userNameValue;

        if(userNameValue &amp;&amp; passwordInput) {
            if (!userNameValue || userNameValue === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || passwordInput.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || emailInvalid) {
                loginButton.disabled = true;
            } else {
                loginButton.disabled = false;
            }
        }
    }

    function backToHome() {
        window.location.href = window.location.origin;
    }
    window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
        setLocale();

        let emailInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        if (emailInput &amp;&amp; emailInput.value.trim()) {
            checkEmailPattern();
        }

        disableLoginButton();

        let url = window.location.href;
        if(url.includes(&quot; , &quot;'&quot; , &quot;execution&quot; , &quot;'&quot; , &quot;)){
            const warningElm = document.querySelector(&quot; , &quot;'&quot; , &quot;.pf-c-alert.pf-m-warning&quot; , &quot;'&quot; , &quot;);
            if (warningElm) {
                document.querySelector(&quot; , &quot;'&quot; , &quot;.pf-c-alert.pf-m-warning&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;; 
            }
        }
    })

     function ShowHideDiv() {
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);

        var mobileDiv = document.getElementById(&quot;mobilegroup&quot;);
        mobileDiv.style.display = byMobile.checked ? &quot;block&quot; : &quot;none&quot;;

        var emailDiv = document.getElementById(&quot;emailgroup&quot;);
        emailDiv.style.display = byMail.checked ? &quot;block&quot; : &quot;none&quot;;

        var mobilLabel = document.getElementById(&quot;numberLabel&quot;);
        var mailLabel = document.getElementById(&quot;mailLabel&quot;);

        mobilLabel.style.color = byMobile.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        mailLabel.style.color = byMail.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        localStorage.setItem(&quot; , &quot;'&quot; , &quot;login_method&quot; , &quot;'&quot; , &quot;, (byMail.checked ? &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;) )

        let usernameInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        usernameInput.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
        mobileInput.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
        usernameinputField.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

        var errmsg = document.getElementById(&quot;errmsg&quot;);
        if(errmsg){
            if(byMail.checked){
                errmsg.innerHTML = &quot; , &quot;'&quot; , &quot;The email or password is incorrect, Please double-check and try again.&quot; , &quot;'&quot; , &quot;;
            }
            if(byMobile.checked){
                errmsg.innerHTML = &quot; , &quot;'&quot; , &quot;The mobile number or password is incorrect, Please double-check and try again.&quot; , &quot;'&quot; , &quot;;
            }
        }
     }



id(&quot;mobile&quot;)&quot;) or . = concat(&quot;


    
        
            
                
                                    
                
                 Back
            
            
                
            
                    
                            
                                عربي
                            
                        
                
        
    
    
        
            
                    
            
            © 2023 Saudi Esports Federation | All rights reserved.
        
        
            
                
                
      

        function InvalidMsg(textbox) {
          if (textbox.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;Required email address&quot; , &quot;'&quot; , &quot;);
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;please enter a valid email address&quot; , &quot;'&quot; , &quot;);
          } else {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get(&quot; , &quot;'&quot; , &quot;redirect_uri&quot; , &quot;'&quot; , &quot;);
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
            if (email) {
              document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;).value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get(&quot; , &quot;'&quot; , &quot;ui_locales&quot; , &quot;'&quot; , &quot;)|| searchParams.get(&quot; , &quot;'&quot; , &quot;kc_locale&quot; , &quot;'&quot; , &quot;) ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + &quot; , &quot;'&quot; , &quot;/user/club-sign&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;?lang=&quot; , &quot;'&quot; , &quot;+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + &quot; , &quot;'&quot; , &quot;&amp;pageUrl=&quot; , &quot;'&quot; , &quot; + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + &quot; , &quot;'&quot; , &quot;/user/forget-password&quot; , &quot;'&quot; , &quot;;
        }

      
        
          
              
                
                    Log In
                    Welcome to Esports
                
               
                  
                      
                        
                        By Mobile Number
                      
                      
                  
                   
                  
                    
                      
                      By Email
                    
                    
                  
                 
                
                  
                    
                    Mobile Number
                    *
                  
                    
                         +966   Afghanistanأفغانستان+93   Albaniaألبانيا+355   Algeriaالجزائر+213   Andorraأندورا+376   Anguillaأنغيلا+1264   Argentinaالأرجنتين+54   Armeniaأرمينيا+374   Arubaأروبا+297   Australiaأستراليا+61   Austriaالنمسا+43   Azerbaijanأذربيجان+994   Bahamasالباهاما+1242   Bahrainالبحرين+973   Bangladeshبنغلاديش+880   Barbadosبربادوس+1246   Belarusروسيا البيضاء+375   Belgiumبلجيكا+32   Belizeبليز+501   Beninبنين+229   Bermudaبرمودا+1441   Bhutanبوتان+975   Boliviaبوليفيا+591   Bosnia and Herzegovinaالبوسنة والهرسك+387   Botswanaبوتسوانا+267   Brazilالبرازيل+55   Brunei Darussalamبروناي دار السلام+673   Bulgariaبلغاريا+359   Burkina Fasoبوركينا فاسو+226   Burundiبوروندي+257   Cambodiaكمبوديا+855   Cameroonالكاميرون+237   Canadaكندا+1   Cape Verdeالرأس الأخضر+238   Central African Republicجمهورية افريقيا الوسطى+236   Chadتشاد+235   Chileتشيلي+56   Chinaالصين+86   Colombiaكولومبيا+57   Comorosجزر القمر+269   Congoالكونغو+242   Costa Ricaكوستا ريكا+506   Côte d&quot; , &quot;'&quot; , &quot;Ivoireكوت ديفوار+225   Croatia (Hrvatska)كرواتيا (هرفاتسكا)+385   Cubaكوبا+53   Cyprusقبرص+357   Czech Republicجمهورية التشيك+420   Denmarkالدنمارك+45   Djiboutiجيبوتي+253   Dominicaدومينيكا+1767   Ecuadorالإكوادور+593   Egyptمصر+20   El Salvadorالسلفادور+503   Equatorial Guineaغينيا الإستوائية+240   Eritreaإريتريا+291   Estoniaاستونيا+372   Ethiopiaأثيوبيا+251   Fijiفيجي+679   Finlandفنلندا+358   Franceفرنسا+33   Gabonالغابون+241   Gambiaغامبيا+220   Georgiaجورجيا+995   Germanyألمانيا+49   Ghanaغانا+233   Greeceاليونان+30   Grenadaغرينادا+1473   Guatemalaغواتيمالا+502   Guineaغينيا+224   Guinea-Bissauغينيا بيساو+245   Guyanaغيانا+592   Haitiهايتي+509   Hondurasهندوراس+504   Hungaryاليونان+36   Icelandأيسلندا+354   Indiaالهند+91   Indonesiaأندونيسيا+62   Iranإيران+98   Iraqالعراق+964   Irelandأيرلندا+353   Italyإيطاليا+39   Jamaicaجامايكا+1876   Japanاليابان+81   Jordanالأردن+962   Kazakhstanكازاخستان+77   Kenyaكينيا+254   Korea (North)كوريا الشمالية+850   Korea (South)كوريا، الجنوبيه+82   Kuwaitالكويت+965   Kyrgyzstanقرغيزستان+996   Laosلاوس+856   Latviaلاتفيا+371   Lebanonلبنان+961   Lesothoليسوتو+266   Liberiaليبيريا+231   Libyaليبيا+218   Liechtensteinليختنشتاين+423   Lithuaniaليتوانيا+370   Luxembourgلوكسمبورغ+352   Macedoniaمقدونيا+389   Madagascarمدغشقر+261   Malawiمالاوي+265   Malaysiaماليزيا+60   Maldivesجزر المالديف+960   Maliمالي+223   Maltaمالطا+356   Marshall Islandsجزر مارشال+692   Mauritaniaموريتانيا+222   Mauritiusموريشيوس+230   Mexicoالمكسيك+52   Micronesiaميكرونيزيا+691   Moldovaمولدوفا+373   Monacoموناكو+377   Mongoliaمنغوليا+976   Moroccoالمغرب+212   Mozambiqueموزمبيق+258   Myanmarميانمار+95   Namibiaناميبيا+264   Nauruناورو+674   Nepalنيبال+977   Netherlandsهولندا+31   New Zealand (Aotearoa)نيوزيلندا (اوتياروا)+64   Nicaraguaنيكاراغوا+505   Nigerالنيجر+227   Norwayالنرويج+47   Omanسلطنة عمان+968   Pakistanباكستان+92   Palauبالاو+680   Palestinian Territoryالأراضي الفلسطينية+970   Panamaبناما+507   Papua New Guineaبابوا غينيا الجديدة+675   Paraguayباراغواي+595   Peruبيرو+51   Philippinesفيلبيني+63   Polandبولندا+48   Portugalالبرتغال+351   Qatarدولة قطر+974   Romaniaرومانيا+40   Russian Federationالاتحاد الروسي+7   Rwandaرواندا+250   Saint Luciaالقديسة لوسيا+1758   Samoaساموا+685   San Marinoسان مارينو+378   Sao Tome and Principeساو تومي وبرنسيبي+239   Saudi Arabiaالمملكة العربية السعودية+966   Senegalالسنغال+221   Serbiaصربيا+381   Seychellesسيشيل+248   Sierra Leoneسيرا ليون+232   Singaporeسنغافورة+65   Slovakiaسلوفاكيا+421   Sloveniaسلوفينيا+386   Solomon Islandsجزر سليمان+677   Somaliaالصومال+252   South Africaجنوب أفريقيا+27   Spainإسبانيا+34   Sri Lankaسيريلانكا+94   Sudanسودان+249   Surinameسورينام+597   Swazilandسوازيلاند+268   Swedenالسويد+46   Switzerlandسويسرا+41   Syriaسوريا+963   Taiwanتايوان+886   Tajikistanطاجيكستان+992   Tanzaniaتنزانيا+255   Thailandتايلاند+66   Timor-Lesteتيمور ليشتي+670   Togoليذهب+228   Tongaتونغا+676   Trinidad and Tobagoترينداد وتوباغو+1868   Tunisiaتونس+216   Turkeyديك رومي+90   Turkmenistanتركمانستان+993   Tuvaluتوفالو+688   Ugandaأوغندا+256   Ukraineأوكرانيا+380   United Arab Emiratesالإمارات العربية المتحدة+971   United Kingdomالمملكة المتحدة+44   United States of Americaالولايات المتحدة+1684   Uruguayأوروغواي+598   Uzbekistanأوزبكستان+998   Vanuatuفانواتي+678   Venezuelaفنزويلا+58   Viet Namفيتنام+84   Yemenاليمن+967   Zambiaزامبيا+260   Zimbabweزيمبابوي+263
                      
                         Invalid mobile number
                  
                

                 

                
                  
                    
                      
                          Email Address
                    *
                  
                      
                         please enter a vaild email
                

                
                  
                    Password*
                  
                  
                
                
                  
                
                
                

                
                    
                    
                    
                    
                          Forgot Password?
                    
                    

                
              

    
       Don’t have account yet? 
      
        Create New Account
       
    
    


    



                


      

        function InvalidMsg(textbox) {
          if (textbox.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;Required email address&quot; , &quot;'&quot; , &quot;);
          } else if (textbox.validity.typeMismatch) {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;please enter a valid email address&quot; , &quot;'&quot; , &quot;);
          } else {
            textbox.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
          }

          return true;
        }

        function getEmail() {
          let searchParams = new URLSearchParams(window.location.search);
          let redirectUrl = searchParams.get(&quot; , &quot;'&quot; , &quot;redirect_uri&quot; , &quot;'&quot; , &quot;);
          if (redirectUrl) {
            url = new URL(redirectUrl);
            searchParams = new URLSearchParams(url.search);
            let email = searchParams.get(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
            if (email) {
              document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;).value = email;
            }
          }
        }

        getEmail();


        function increaseHistory() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            count = -2;
            localStorage.setItem(&quot;count&quot;, count);
          } else {
            let increasedCount = parseInt(count) - 1;
            localStorage.setItem(&quot;count&quot;, increasedCount);
          }
        }

        function goBack() {
          let count = localStorage.getItem(&quot;count&quot;);
          if (!count) {
            history.back();
          } else {
            history.go((parseInt(count)));
          }
          localStorage.removeItem(&quot;count&quot;);
        } 

       function redirectToCreateAccount() { 
          let searchParams = new URLSearchParams(window.location.search);
          let ui_locales = searchParams.get(&quot; , &quot;'&quot; , &quot;ui_locales&quot; , &quot;'&quot; , &quot;)|| searchParams.get(&quot; , &quot;'&quot; , &quot;kc_locale&quot; , &quot;'&quot; , &quot;) ||localStorage.getItem(&quot;lang&quot;);
          /*
            pass redirect_uri to signup page to enable redirects after signup
            Check if pageURL param exists trim protocol and domain from the redirect_uri
            and take only pathname to be passed to pageURL
          */
          let redirectURI = searchParams.get(&quot;redirect_uri&quot;);
          let fullURL =   window.location.origin + &quot; , &quot;'&quot; , &quot;/user/club-sign&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;?lang=&quot; , &quot;'&quot; , &quot;+ui_locales;

          if (redirectURI) {
            fullURL = fullURL + &quot; , &quot;'&quot; , &quot;&amp;pageUrl=&quot; , &quot;'&quot; , &quot; + redirectURI.split(&quot;saudiesports.sa&quot;).pop().split(&quot;localhost:4200&quot;).pop();
          }
            window.location.href = fullURL;
          }
        function redirectToForgetPassword() {
           window.location.href = window.location.origin + &quot; , &quot;'&quot; , &quot;/user/forget-password&quot; , &quot;'&quot; , &quot;;
        }

      





                
            
        
    




    const setLocale = () => {
        &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            localStorage.setItem(&quot; , &quot;'&quot; , &quot;lang&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;);
        &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
    }

    var selectedMobileCode = &quot; , &quot;'&quot; , &quot;+966&quot; , &quot;'&quot; , &quot;;
    var selectedMobilePattern= new RegExp(&quot;^(05|5)(5|0|3|6|4|9|1|8|7)([0-9]{7})$&quot;);
    function getData() {

      const loginMethod = localStorage.getItem(&quot; , &quot;'&quot; , &quot;login_method&quot; , &quot;'&quot; , &quot;) || &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;;
        document.getElementById(&quot;byMail&quot;).checked = (loginMethod === &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot;byNumber&quot;).checked = (loginMethod !== &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;) ;
        ShowHideDiv();
      fetch(&quot; , &quot;'&quot; , &quot;/api/team-club/v1/public/geography/country&quot; , &quot;'&quot; , &quot;)
        .then(response => response.json())
        .then(data => {
        const listContainer = document.getElementById(&quot;dropdown&quot;);
        const dropdownbtn = document.createElement(&quot;div&quot;);
        dropdownbtn.className =&quot;dropbtn arrow&quot;;
        dropdownbtn.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;defult-country-flag&quot;)

        const searchplaceholder  = &quot; , &quot;'&quot; , &quot;Search..&quot; , &quot;'&quot; , &quot;;

        dropdownbtn.innerHTML = &quot;&lt;img src=&quot; , &quot;'&quot; , &quot;/resource/esports/public/sa.png&quot; , &quot;'&quot; , &quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp +966&quot;;
        listContainer.appendChild(dropdownbtn);

        const listdiv = document.createElement(&quot;div&quot;);
        listdiv.className = &quot;dropdown-content&quot;
        listdiv.id =&quot;myDropdown&quot;
        listContainer.appendChild(listdiv);

         const searchInput = document.createElement(&quot;input&quot;);
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, &quot;text&quot;);
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;myInput&quot;)
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;, searchplaceholder)
         searchInput.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, &quot;search-field&quot;)
         listdiv.appendChild(searchInput);
         dropdownbtn.addEventListener(&quot;click&quot;, () => myFunction());

         searchInput.addEventListener(&quot;keyup&quot;, () => filterFunction());

          data.forEach((item,index) => {
            const listItem = document.createElement(&quot;a&quot;);
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, &quot;#&quot;);
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;, &quot;country-flag-&quot; + index)
            listItem.setAttribute(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;, &quot;return false ;&quot;);
            listItem.onclick = function(){return false};
            listdiv.appendChild(listItem);

            const countryCode = document.createElement(&quot;span&quot;);
            countryCode.innerText = item.dialCode;
            countryCode.className =&quot;country-code&quot;;

            document.getElementById(&quot;country-flag-&quot; + index).innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ &quot; , &quot;'&quot; , &quot;/resource/esports/public/&quot; , &quot;'&quot; , &quot; + item.code + &quot; , &quot;'&quot; , &quot;.png&quot; , &quot;'&quot; , &quot;+&quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp &lt;span style=&quot; , &quot;'&quot; , &quot;display:none;&quot; , &quot;'&quot; , &quot;>&quot; +item.name + item.nameAr +&quot;&lt;/span>&quot;;
            listItem.appendChild(countryCode);
            listItem.addEventListener(&quot;click&quot;, () => handleItemClick(item.code, item.flag , item.dialCode,item.regex));
          });
        })
        .catch(error => console.error(error));
    }

    function myFunction() {
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
    }

    function handleItemClick(countryCode , flag , mbileCode,regex){
      document.getElementById(&quot;myDropdown&quot;).classList.toggle(&quot;show&quot;);
      const dropdownbtn= document.getElementById(&quot;defult-country-flag&quot;);
      selectedMobileCode = mbileCode;
      selectedMobilePattern = new RegExp(regex);
      let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
      var byMobile = document.getElementById(&quot;byNumber&quot;);
      let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
      if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
            usernameinputField.value = userNameValue;
        }
      dropdownbtn.innerHTML = &quot;&lt;img src=&quot;+window.location.origin+ &quot; , &quot;'&quot; , &quot;/resource/esports/public/&quot; , &quot;'&quot; , &quot; + countryCode + &quot; , &quot;'&quot; , &quot;.png&quot; , &quot;'&quot; , &quot;+&quot; width=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;25&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;country-flag&quot; , &quot;'&quot; , &quot;/> &amp;nbsp&quot;+ mbileCode;
    }

    function filterFunction() {
      var input, filter, ul, li, a, i;
      input = document.getElementById(&quot;myInput&quot;);
      filter = input.value.toUpperCase();
      div = document.getElementById(&quot;myDropdown&quot;);
      a = div.getElementsByTagName(&quot;a&quot;);
      for (i = 0; i &lt; a.length; i++) {
        txtValue = a[i].textContent || a[i].innerText;
        if (txtValue.toUpperCase().indexOf(filter) > -1) {
          a[i].style.display = &quot;&quot;;
        } else {
          a[i].style.display = &quot;none&quot;;
        }
      }
    }


    let emailInvalid = true;

        function checkMobilePattern() {
            const elem = document.getElementById(&quot;mobile&quot;);
            const regex = selectedMobilePattern;
            const inputControl = elem.parentElement;
            const errorDisplay = inputControl.querySelector(&quot; , &quot;'&quot; , &quot;.error-msg-email&quot; , &quot;'&quot; , &quot;);
            if (!regex.test(elem.value)) {
                errorDisplay.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
                emailInvalid = true;
            } else {
                errorDisplay.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
                emailInvalid = false;
            }
            disableLoginButton();
        }

    function checkEmailPattern() {
        const elem = document.getElementById(&quot;email&quot;);
        const regex = /^(([^&lt;>()[\]\.,;:\s@\&quot;]+(\.[^&lt;>()[\]\.,;:\s@\&quot;]+)*)|(\&quot;.+\&quot;))@(([^&lt;>()[\]\.,;:\s@\&quot;]+\.)+[^&lt;>()[\]\.,;:\s@\&quot;]{2,})$/i;
        const inputControl = elem.parentElement;
        const errorDisplay = inputControl.querySelector(&quot; , &quot;'&quot; , &quot;.error-msg-email&quot; , &quot;'&quot; , &quot;);
        if (!regex.test(elem.value)) {
            errorDisplay.classList.remove(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
            emailInvalid = true;
        } else {
            errorDisplay.classList.add(&quot; , &quot;'&quot; , &quot;d-none&quot; , &quot;'&quot; , &quot;);
            emailInvalid = false;
        }
        disableLoginButton();
    }

    // Disable login button if has not value
    function disableLoginButton() {
        let usernameInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
        let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
        let passwordInput = document.getElementById(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
        let loginButton = document.getElementById(&quot; , &quot;'&quot; , &quot;kc-login&quot; , &quot;'&quot; , &quot;);
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);
        let userNameValue ;
        if(byMobile.checked ){
            userNameValue = selectedMobileCode + &quot;&quot; +  mobileInput.value ;
        }
        if(byMail.checked ){
            userNameValue = usernameInput.value;
        }
         usernameinputField.value = userNameValue;

        if(userNameValue &amp;&amp; passwordInput) {
            if (!userNameValue || userNameValue === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || passwordInput.value === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || emailInvalid) {
                loginButton.disabled = true;
            } else {
                loginButton.disabled = false;
            }
        }
    }

    function backToHome() {
        window.location.href = window.location.origin;
    }
    window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
        setLocale();

        let emailInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        if (emailInput &amp;&amp; emailInput.value.trim()) {
            checkEmailPattern();
        }

        disableLoginButton();

        let url = window.location.href;
        if(url.includes(&quot; , &quot;'&quot; , &quot;execution&quot; , &quot;'&quot; , &quot;)){
            const warningElm = document.querySelector(&quot; , &quot;'&quot; , &quot;.pf-c-alert.pf-m-warning&quot; , &quot;'&quot; , &quot;);
            if (warningElm) {
                document.querySelector(&quot; , &quot;'&quot; , &quot;.pf-c-alert.pf-m-warning&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;; 
            }
        }
    })

     function ShowHideDiv() {
        var byMail = document.getElementById(&quot;byMail&quot;);
        var byMobile = document.getElementById(&quot;byNumber&quot;);

        var mobileDiv = document.getElementById(&quot;mobilegroup&quot;);
        mobileDiv.style.display = byMobile.checked ? &quot;block&quot; : &quot;none&quot;;

        var emailDiv = document.getElementById(&quot;emailgroup&quot;);
        emailDiv.style.display = byMail.checked ? &quot;block&quot; : &quot;none&quot;;

        var mobilLabel = document.getElementById(&quot;numberLabel&quot;);
        var mailLabel = document.getElementById(&quot;mailLabel&quot;);

        mobilLabel.style.color = byMobile.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        mailLabel.style.color = byMail.checked ? &quot;white&quot; : &quot;#9FAAB3&quot;;
        localStorage.setItem(&quot; , &quot;'&quot; , &quot;login_method&quot; , &quot;'&quot; , &quot;, (byMail.checked ? &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;) )

        let usernameInput = document.getElementById(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
        usernameInput.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        let mobileInput = document.getElementById(&quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;);
        mobileInput.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        let usernameinputField = document.getElementById(&quot; , &quot;'&quot; , &quot;usernameinput&quot; , &quot;'&quot; , &quot;);
        usernameinputField.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;

        var errmsg = document.getElementById(&quot;errmsg&quot;);
        if(errmsg){
            if(byMail.checked){
                errmsg.innerHTML = &quot; , &quot;'&quot; , &quot;The email or password is incorrect, Please double-check and try again.&quot; , &quot;'&quot; , &quot;;
            }
            if(byMobile.checked){
                errmsg.innerHTML = &quot; , &quot;'&quot; , &quot;The mobile number or password is incorrect, Please double-check and try again.&quot; , &quot;'&quot; , &quot;;
            }
        }
     }



id(&quot;mobile&quot;)&quot;))]</value>
      <webElementGuid>b1757c2d-bd34-41f4-9e8d-019a772d1cec</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
