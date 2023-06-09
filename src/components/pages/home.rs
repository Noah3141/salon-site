use gloo::utils::window;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    let doc = document.clone();
    let top = Callback::from(move |_: MouseEvent| {
        doc
            .get_element_by_id("top")
            .unwrap()
            .scroll_into_view();
    });
    
    let doc = document.clone();
    let contact_us = Callback::from(move |_: MouseEvent| {
        doc
            .get_element_by_id("contact")
            .unwrap()
            .scroll_into_view();

    });
   

    html! {
        <>
        <header id="top"><img class="bg-fixed bg-contain" id="top" src="../images/woman.jpg" alt="Woman"/></header>

        <section id="about" class="font-body text-xl">
            <div class="bg-paper-back w-full px-12 lg:px-32 py-32 bg-zinc-50">
                // Row
                <div class="container lg:grid grid-cols-2 gap-2">
                    // Left hand side
                    <div class=" px-4 sm:pb-4">
                        <img 
                            src="../images/SalonRow.jpg" 
                            alt="Row of Salon Chairs"
                            class="border-8 border-[#3f3f46] w-full"
                        />
                    
                    </div>
                    // Right hand side
                    <div class="px-4 ">
                        <h1 class="title text-zinc-800 pb-6 pt-4 md:pt-0">{"WELCOME"}</h1>
                        <p class="text-zinc-600">{"Our goal at Salon is to give each client an exceptional hair design that reflects their unique personality and compliments their lifestyle. We view hair as an art. Each style is suited specifically to the client's request, and is composed with practiced hands and delivered with outstanding expertise. "}</p>
                        
                        <div class="flex flex-row gap-6 pt-8">
                            <svg class="fill-rose-400 cursor-pointer hover:fill-rose-500" xmlns="http://www.w3.org/2000/svg" width="38" height="38"  class="bi bi-facebook" viewBox="0 0 16 16">
                                <path d="M16 8.049c0-4.446-3.582-8.05-8-8.05C3.58 0-.002 3.603-.002 8.05c0 4.017 2.926 7.347 6.75 7.951v-5.625h-2.03V8.05H6.75V6.275c0-2.017 1.195-3.131 3.022-3.131.876 0 1.791.157 1.791.157v1.98h-1.009c-.993 0-1.303.621-1.303 1.258v1.51h2.218l-.354 2.326H9.25V16c3.824-.604 6.75-3.934 6.75-7.951z"/>
                            </svg>
                            <svg class="fill-rose-400 cursor-pointer hover:fill-rose-500" xmlns="http://www.w3.org/2000/svg" width="38" height="38"  class="bi bi-instagram" viewBox="0 0 16 16">
                                <path d="M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.917 3.917 0 0 0-1.417.923A3.927 3.927 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.916 3.916 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.926 3.926 0 0 0-.923-1.417A3.911 3.911 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0h.003zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599.28.28.453.546.598.92.11.281.24.705.275 1.485.039.843.047 1.096.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.47 2.47 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.478 2.478 0 0 1-.92-.598 2.48 2.48 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.233 0-2.136.008-2.388.046-3.231.036-.78.166-1.204.276-1.486.145-.373.319-.64.599-.92.28-.28.546-.453.92-.598.282-.11.705-.24 1.485-.276.738-.034 1.024-.044 2.515-.045v.002zm4.988 1.328a.96.96 0 1 0 0 1.92.96.96 0 0 0 0-1.92zm-4.27 1.122a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217zm0 1.441a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334z"/>
                            </svg>
                            <svg class="fill-rose-400 cursor-pointer hover:fill-rose-500" xmlns="http://www.w3.org/2000/svg" width="38" height="38"  class="bi bi-pinterest" viewBox="0 0 16 16">
                                <path d="M8 0a8 8 0 0 0-2.915 15.452c-.07-.633-.134-1.606.027-2.297.146-.625.938-3.977.938-3.977s-.239-.479-.239-1.187c0-1.113.645-1.943 1.448-1.943.682 0 1.012.512 1.012 1.127 0 .686-.437 1.712-.663 2.663-.188.796.4 1.446 1.185 1.446 1.422 0 2.515-1.5 2.515-3.664 0-1.915-1.377-3.254-3.342-3.254-2.276 0-3.612 1.707-3.612 3.471 0 .688.265 1.425.595 1.826a.24.24 0 0 1 .056.23c-.061.252-.196.796-.222.907-.035.146-.116.177-.268.107-1-.465-1.624-1.926-1.624-3.1 0-2.523 1.834-4.84 5.286-4.84 2.775 0 4.932 1.977 4.932 4.62 0 2.757-1.739 4.976-4.151 4.976-.811 0-1.573-.421-1.834-.919l-.498 1.902c-.181.695-.669 1.566-.995 2.097A8 8 0 1 0 8 0z"/>
                            </svg>
                            <div onclick={contact_us} class="button text-xs pt-1 sm:pt-2 sm:text-sm">{"GET IN TOUCH"}</div>
                        </div>
                    </div>

                </div>
            </div>
        </section>


        <section id="reviews" class="relative font-body text-xl h-full">
            <div class="bg-sidebar-back bg-fixed bg-cover text-zinc-200 px-12 md:px-32 py-16">
                <div class="">
                    // Title
                    <h1 class="title text-center mb-16">{"REVIEWS"}</h1>
                    // First
                <div class="px-4 my-4 md:w-2/3 font-body">
                        <p class="">
                            <i>{"I love this salon! Everyone there is wonderful! I get expert coloring and expert cutting! I would highly recommend this salon to anyone! I was just there and someone had called needing to get her hair done at the last minute for a wedding, the stylists said come on In! Every other salon she called said no."}</i>
                        </p>
                        <p class="">
                            {"— Jennifer"}
                        </p>
                    </div>
                    // Second
                <div class="px-4 my-4 md:w-2/3 font-body">
                        <p class="">
                            <i>{"I've tried many salons in this city and this salon is the overall best value. Do yourself a favor and go to this salon first!"}</i>
                        </p>
                        <p class="">
                            {"— Samantha"}
                        </p>
                    </div>
                    //Third
                <div class="px-4 my-4 md:w-2/3 font-body">
                        <p class="">
                            <i>{"Amazing! I don't think I will ever get my hair cut anywhere else. Always impressed with the results and friendly service, plus a super convenient new location in the city! Highly recommended!"}</i>
                        </p>
                        <p class="">
                            {"— Andrea"}
                        </p>
                    </div>
                </div>
            </div>
        </section>



        <section id="services" class="text-zinc-800 ">
            <div class="bg-grid-back bg-cover px-12 md:px-32 py-16">
                <h1 class="title text-center text-zinc-800 mb-16">{"SERVICES"}</h1>

                <table class="mx-0 px-0 sm:mx-4 sm:px-4 text-lg sm:text-2xl font-body border-separate border-spacing-y-6 sm:border-spacing-y-2">
                    <tbody class="">
                        <tr class="">
                            <td>{"Women's Haircut"}</td>
                            <td>{"$55 and up"}</td>
                        </tr>
                            <span class="text-sm">
                            {"Longer or thicker hair takes longer work with, and starts at $65"}
                            </span>

                        <tr class="">
                            <td>{"Men's Haircut"}</td>
                            <td>{"$30 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="">
                            <td>{"Haircuts"}</td>
                            <td>{"$55 and up"}</td>
                            <td></td>
                        </tr>

                        <div class="my-6 w-full"></div>

                        <span class="pr-8">{"Coloring with blow dry:"}</span>
                        <tr class="">
                            <td>{"Touch Up"}</td>
                            <td>{"$85 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="">
                            <td>{"Partial Highlight"}</td>
                            <td>{"$115 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="">
                            <td>{"Full Highlight"}</td>
                            <td>{"$140 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="">
                            <td>{"Sunkissed Bayalage"}</td>
                            <td>{"$145 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="">
                            <td>{"Customized Color"}</td>
                            <td>{"To be determined at service"}</td>
                            <td></td>
                        </tr>

                        <div class="my-3 w-full"></div>

                        <tr class="py-4">
                            <td>{"Shampoo/Blowdry"}</td>
                            <td>{"$30 and up"}</td>
                            <td></td>
                        </tr>
                        <tr class="py-4">
                            <td>{"Shampoo/Blowdry/Soft Waves"}</td>
                            <td>{"$35"}</td>
                            <td></td>
                        </tr>
                        <tr class="py-4">
                            <td>{"Bridal Hair"}</td>
                            <td>{"$400"}</td>
                            <td></td>
                        </tr>

                        <div class="my-3 w-full"></div>

                        <tr class="py-4">
                            <td>{"Express Keratin"}</td>
                            <td>{"$95"}</td>
                            <td></td>
                        </tr>
                        <tr class="py-4">
                            <td>{"Keratin"}</td>
                            <td>{"$200"}</td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>


            </div>
        </section>



        <section id="stylists" class="text-zinc-200 ">
            <div class="bg-bottombar-back bg-cover px-12 md:px-32 pt-16 pb-96">
                <h1 class="title text-center">{"STYLISTS"}</h1>
                
                <div class="pt-4 grid grid-cols-2"> // GRID
                    // USE CHANGES IN THE BREAKPOINT TO ALLOW FOR VARIOUS SIZES OF TEXT IN THE P TAG
                    <div class="col-span-2 xl:col-span-1 m-4 w-full " > // CELL - ROW INTERNALLY
                        <div class="flex flex-col sm:flex-row justify-start">
                        <img class="m-0" style="height: 270px" src="../images/NoProfilePhoto.svg" alt="" /> // LEFT
                        <div class="px-4 min-w-96">  // RIGHT
                            <h2 class="title">{"Andrea"}</h2>
                            <p class="font-body ">
                            {"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Lorem ipsum dolor sit amet, consectetur adipisicing elit. Perspiciatis id, voluptas laborum velit eius dolorem iure consequuntur dolores molestias maiores doloribus possimus! Nisi accusamus error ut inventore aspernatur libero cupiditate."}
                            </p>
                        </div>
                        </div>
                    </div> 

                    <div class="col-span-2 xl:col-span-1 m-4 w-full " >
                        <div class="flex flex-col sm:flex-row content-start">
                        <img class="m-0" style="height: 270px" src="../images/NoProfilePhoto.svg" alt="" />
                        <div class="px-4 min-w-96">
                            <h2 class="title">{"Deandra"}</h2>
                            <p class="font-body ">
                            {"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Perspiciatis id, voluptas laborum velit eius dolorem iure consequuntur dolores molestias maiores doloribus possimus! Nisi accusamus error ut inventore aspernatur libero cupiditate."}
                            </p>
                            <div class="button mt-6 w-56 py-2 text-center">{"BOOK WITH DEANDRA"}</div>
                        </div>
                        </div>
                    </div>
                
                    <div class="col-span-2 xl:col-span-1 m-4 w-full " >
                        <div class="flex flex-col sm:flex-row content-start">
                        <img class="m-0" style="height: 270px" src="../images/NoProfilePhoto.svg" alt="" />
                        <div class="px-4 min-w-96">
                            <h2 class="title">{"Deedee"}</h2>
                            <p class="font-body ">
                            {"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Perspiciatis id, voluptas laborum velit eius dolorem iure consequuntur dolores molestias maiores doloribus possimus! Nisi accusamus error ut inventore aspernatur libero cupiditate."}
                            </p>
                            <div class="button mt-6 w-56 py-2 text-center">{"BOOK WITH DEEDEE"}</div>
                        </div>
                        </div>
                    </div>

                    <div class="col-span-2 xl:col-span-1 m-4 w-full " >
                        <div class="flex flex-col sm:flex-row content-start">
                        <img class="m-0" style="height: 270px" src="../images/NoProfilePhoto.svg" alt="" />
                        <div class="px-4 min-w-96">
                            <h2 class="title">{"Alexandria"}</h2>
                            <p class="font-body ">
                            {"Lorem ipsum dolor sit amet, consectetur adipisicing elit. Perspiciatis id, voluptas laborum velit eius dolorem iure consequuntur dolores molestias maiores doloribus possimus! Nisi accusamus error ut inventore aspernatur libero cupiditate."}
                            </p>
                            <div class="button mt-6 w-56 py-2 text-center">{"BOOK WITH ALEXANDRIA"}</div>
                        </div>
                        </div>
                    </div>

                    </div>
                    <span id="contact"></span>
            </div>
        </section>
        <section  class="bg-rose-400">
            <div class="flex flex-col sm:flex-row pb-8 text-zinc-800 justify-center">
                <div class="pe-4  sm:pe-48 font-body text-center">
                    <h1 class="title">{"CONTACT US"}</h1>
                    <div>
                        <p>{"706-354-0054"}</p>
                        <p>{"flourishhair2015@gmail.com"}</p>
                        
                        <p class="mt-4">{"630 Olympic Drive Athens Ga 30606"}</p>
                        <a href="https://goo.gl/maps/tMAm7tHtKaZr32U67"><p class="font-headliner hover:text-zinc-900 cursor-pointer">{"MAP"}</p></a>
                        
                        <p class="mt-4">{"Monday - Friday 8am to 6pm"}</p>
                        <p>{"Saturday - 10am to 2pm"}</p>
                        <p>{"Sunday - Closed"}</p>
                    </div>
                </div>
                <div class="mt-4 sm:mt-0 mx-auto sm:mx-0 justify-center content-center">
                    <img 
                        class="border-4 border-zinc-700" 
                        width="400px" 
                        height="200px"
                        src="../images/Salon-Lofts-storefront.jpg" 
                        alt="Storefront" 
                    />
                </div>
            </div>
        </section>

        <img 
            src="../images/transition-pink-to-dark.svg" 
            alt=""
            class="w-full -mt-1" 
        />

        <section id="gallery" class="">
            <div class="bg-zinc-700 py-16 -mt-1">
                <h1 class="title text-zinc-200 text-center">{"Gallery"}</h1>

                <div class="transition-all duration-100 grid grid-cols-12 gap-3 mt-8 px-12 md:px-32">
                
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                <img class="col-span-12 sm:col-span-6 lg:col-span-4 xl:col-span-3 w-full" src="../images/NoProfilePhoto.svg" alt="" />
                
                </div>
            </div>
        </section>
        
        
        
        

        <footer class="py-5 bg-zinc-800">
            <div class="flex flex-row">
                <div class="flex w-1/2 justify-start"> // Just use "justify-content: space-between" next time, damn
                    <span class="px-16 text-zinc-400">{"© Flourish Salon Athens, Georgia"}</span>
                </div>
                <div class="flex w-1/2 justify-end">
                
                    <div class="flex gap-4 pe-20">
                        <svg class="fill-zinc-400 cursor-pointer hover:fill-zinc-500" xmlns="http://www.w3.org/2000/svg" width="32" height="32"  class="bi bi-facebook" viewBox="0 0 16 16">
                        <path d="M16 8.049c0-4.446-3.582-8.05-8-8.05C3.58 0-.002 3.603-.002 8.05c0 4.017 2.926 7.347 6.75 7.951v-5.625h-2.03V8.05H6.75V6.275c0-2.017 1.195-3.131 3.022-3.131.876 0 1.791.157 1.791.157v1.98h-1.009c-.993 0-1.303.621-1.303 1.258v1.51h2.218l-.354 2.326H9.25V16c3.824-.604 6.75-3.934 6.75-7.951z"/>
                        </svg>
                        <svg class="fill-zinc-400 cursor-pointer hover:fill-zinc-500" xmlns="http://www.w3.org/2000/svg" width="32" height="32"  class="bi bi-instagram" viewBox="0 0 16 16">
                            <path d="M8 0C5.829 0 5.556.01 4.703.048 3.85.088 3.269.222 2.76.42a3.917 3.917 0 0 0-1.417.923A3.927 3.927 0 0 0 .42 2.76C.222 3.268.087 3.85.048 4.7.01 5.555 0 5.827 0 8.001c0 2.172.01 2.444.048 3.297.04.852.174 1.433.372 1.942.205.526.478.972.923 1.417.444.445.89.719 1.416.923.51.198 1.09.333 1.942.372C5.555 15.99 5.827 16 8 16s2.444-.01 3.298-.048c.851-.04 1.434-.174 1.943-.372a3.916 3.916 0 0 0 1.416-.923c.445-.445.718-.891.923-1.417.197-.509.332-1.09.372-1.942C15.99 10.445 16 10.173 16 8s-.01-2.445-.048-3.299c-.04-.851-.175-1.433-.372-1.941a3.926 3.926 0 0 0-.923-1.417A3.911 3.911 0 0 0 13.24.42c-.51-.198-1.092-.333-1.943-.372C10.443.01 10.172 0 7.998 0h.003zm-.717 1.442h.718c2.136 0 2.389.007 3.232.046.78.035 1.204.166 1.486.275.373.145.64.319.92.599.28.28.453.546.598.92.11.281.24.705.275 1.485.039.843.047 1.096.047 3.231s-.008 2.389-.047 3.232c-.035.78-.166 1.203-.275 1.485a2.47 2.47 0 0 1-.599.919c-.28.28-.546.453-.92.598-.28.11-.704.24-1.485.276-.843.038-1.096.047-3.232.047s-2.39-.009-3.233-.047c-.78-.036-1.203-.166-1.485-.276a2.478 2.478 0 0 1-.92-.598 2.48 2.48 0 0 1-.6-.92c-.109-.281-.24-.705-.275-1.485-.038-.843-.046-1.096-.046-3.233 0-2.136.008-2.388.046-3.231.036-.78.166-1.204.276-1.486.145-.373.319-.64.599-.92.28-.28.546-.453.92-.598.282-.11.705-.24 1.485-.276.738-.034 1.024-.044 2.515-.045v.002zm4.988 1.328a.96.96 0 1 0 0 1.92.96.96 0 0 0 0-1.92zm-4.27 1.122a4.109 4.109 0 1 0 0 8.217 4.109 4.109 0 0 0 0-8.217zm0 1.441a2.667 2.667 0 1 1 0 5.334 2.667 2.667 0 0 1 0-5.334z"/>
                        </svg>
                        <svg class="fill-zinc-400 cursor-pointer hover:fill-zinc-500" xmlns="http://www.w3.org/2000/svg" width="32" height="32"  class="bi bi-pinterest" viewBox="0 0 16 16">
                            <path d="M8 0a8 8 0 0 0-2.915 15.452c-.07-.633-.134-1.606.027-2.297.146-.625.938-3.977.938-3.977s-.239-.479-.239-1.187c0-1.113.645-1.943 1.448-1.943.682 0 1.012.512 1.012 1.127 0 .686-.437 1.712-.663 2.663-.188.796.4 1.446 1.185 1.446 1.422 0 2.515-1.5 2.515-3.664 0-1.915-1.377-3.254-3.342-3.254-2.276 0-3.612 1.707-3.612 3.471 0 .688.265 1.425.595 1.826a.24.24 0 0 1 .056.23c-.061.252-.196.796-.222.907-.035.146-.116.177-.268.107-1-.465-1.624-1.926-1.624-3.1 0-2.523 1.834-4.84 5.286-4.84 2.775 0 4.932 1.977 4.932 4.62 0 2.757-1.739 4.976-4.151 4.976-.811 0-1.573-.421-1.834-.919l-.498 1.902c-.181.695-.669 1.566-.995 2.097A8 8 0 1 0 8 0z"/>
                        </svg>
                    </div>
                    
                </div>
            </div>
        </footer>


        <div onclick={top} class="w-10 h-10 shadow bg-zinc-500 hover:bg-zinc-900 rounded-full cursor-pointer fixed text-center font-bold bottom-4 right-6">
            <img  class="m-auto mt-3" height="12px" width="12px" src="../images/ScrollTopIcon.svg" alt="Top"/>
        </div>
        </>
    }
}