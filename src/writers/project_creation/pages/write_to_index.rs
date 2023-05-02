use std::fs::OpenOptions;
use std::io::{Error, Write};
use crate::Project;

// Write to index.html.tera
pub fn write_to_index_html(project: &Project) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&project.index_html)
        .expect("Failed to open index.html");

    file.write_all(
        b"{% extends 'base.html.tera' %}
{% block title %}Index{% endblock title %}
{% block head %}
{{ super() }}
{% endblock head %}
{% block content %}
<div class='bg-gray-900 pt-10 sm:pt-16 lg:overflow-hidden lg:pt-8 lg:pb-14'>
  <div class='mx-auto max-w-7xl lg:px-8'>
    <div class='lg:grid lg:grid-cols-2 lg:gap-8'>
      <div class='mx-auto max-w-md px-6 sm:max-w-2xl sm:text-center lg:flex lg:items-center lg:px-0 lg:text-left'>
        <div class='lg:py-24'>
          <a href='#'
            class='inline-flex items-center rounded-full bg-black p-1 pr-2 text-white hover:text-gray-200 sm:text-base lg:text-sm xl:text-base'>
            <span
              class='rounded-full bg-gradient-to-r from-teal-500 to-cyan-600 px-3 py-0.5 text-sm font-semibold leading-5 text-white'>We\xE2\x80\x99re
              hiring</span>
            <span class='ml-4 text-sm'>Visit our careers page</span>
            <!-- Heroicon name: mini/chevron-right -->
            <svg class='ml-2 h-5 w-5 text-gray-500' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'
              fill='currentColor' aria-hidden='true'>
              <path fill-rule='evenodd'
                d='M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z'
                clip-rule='evenodd' />
            </svg>
          </a>
          <h1 class='mt-4 text-4xl font-bold tracking-tight text-white sm:mt-5 sm:text-6xl lg:mt-6 xl:text-6xl'>
            <span class='block'>A better way to</span>
            <span
              class='block bg-gradient-to-r from-teal-200 to-cyan-400 bg-clip-text pb-3 text-transparent sm:pb-5'>ship
              web apps</span>
          </h1>
          <p class='text-base text-gray-300 sm:text-xl lg:text-lg xl:text-xl'>Anim aute id magna aliqua ad ad non
            deserunt sunt. Qui irure qui Lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat fugiat.</p>
          <div class='mt-10 sm:mt-12'>
            <form action='#' class='sm:mx-auto sm:max-w-xl lg:mx-0'>
              <div class='sm:flex'>
                <div class='min-w-0 flex-1'>
                  <label for='email' class='sr-only'>Email address</label>
                  <input id='email' type='email' placeholder='Enter your email'
                    class='block w-full rounded-md border-0 px-4 py-3 text-base text-gray-900 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-offset-2 focus:ring-offset-gray-900'>
                </div>
                <div class='mt-3 sm:mt-0 sm:ml-3'>
                  <button type='submit'
                    class='block w-full rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 py-3 px-4 font-medium text-white shadow hover:from-teal-600 hover:to-cyan-700 focus:outline-none focus:ring-2 focus:ring-cyan-400 focus:ring-offset-2 focus:ring-offset-gray-900'>Start
                    free trial</button>
                </div>
              </div>
              <p class='mt-3 text-sm text-gray-300 sm:mt-4'>Start your free 14-day trial, no credit card necessary. By
                providing your email, you agree to our <a href='#' class='font-medium text-white'>terms of service</a>.
              </p>
            </form>
          </div>
        </div>
      </div>
      <div class='mt-12 -mb-16 sm:-mb-48 lg:relative lg:m-0'>
        <div class='mx-auto max-w-md px-6 sm:max-w-2xl lg:max-w-none lg:px-0'>
          <!-- Illustration taken from Lucid Illustrations: https://lucid.pixsellz.io/ -->
          <img class='w-full lg:absolute lg:inset-y-0 lg:left-0 lg:h-full lg:w-auto lg:max-w-none'
            src='https://tailwindui.com/img/component-images/cloud-illustration-teal-cyan.svg' alt=''>
        </div>
      </div>
    </div>
  </div>
</div>

<main>
  <!-- Feature section with screenshot -->
  <!-- Ignore if not present -->
  {% include 'sections/feature-section-w-screenshot'  ignore missing %}

  <!-- Feature section with grid -->
  <div class='relative bg-white py-16 sm:py-24 lg:py-32'>
    <div class='mx-auto max-w-md px-6 text-center sm:max-w-3xl lg:max-w-7xl lg:px-8'>
      <h2 class='text-lg font-semibold text-cyan-600'>Deploy faster</h2>
      <p class='mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl'>Everything you need to deploy your app
      </p>
      <p class='mx-auto mt-5 max-w-prose text-xl text-gray-500'>Phasellus lorem quam molestie id quisque diam aenean nulla
        in. Accumsan in quis quis nunc, ullamcorper malesuada. Eleifend condimentum id viverra nulla.</p>
      <div class='mt-12'>
        <div class='grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3'>
          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/cloud-arrow-up -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Push to Deploy</h3>
                <p class='mt-5 text-base text-gray-500'>Ac tincidunt sapien vehicula erat auctor pellentesque rhoncus. Et
                  magna sit morbi vitae lobortis.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/lock-closed -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>SSL Certificates</h3>
                <p class='mt-5 text-base text-gray-500'>Qui aut temporibus nesciunt vitae dicta repellat sit dolores
                  pariatur. Temporibus qui illum aut.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/arrow-path -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M4.5 12c0-1.232.046-2.453.138-3.662a4.006 4.006 0 013.7-3.7 48.678 48.678 0 017.324 0 4.006 4.006 0 013.7 3.7c.017.22.032.441.046.662M4.5 12l-3-3m3 3l3-3m12 3c0 1.232-.046 2.453-.138 3.662a4.006 4.006 0 01-3.7 3.7 48.657 48.657 0 01-7.324 0 4.006 4.006 0 01-3.7-3.7c-.017-.22-.032-.441-.046-.662M19.5 12l-3 3m3-3l3 3' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Simple Queues</h3>
                <p class='mt-5 text-base text-gray-500'>Rerum quas incidunt deleniti quaerat suscipit mollitia. Amet
                  repellendus ut odit dolores qui.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/shield-check -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Advanced Security</h3>
                <p class='mt-5 text-base text-gray-500'>Ullam laboriosam est voluptatem maxime ut mollitia commodi. Et
                  dignissimos suscipit perspiciatis.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/cog -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M4.5 12a7.5 7.5 0 0015 0m-15 0a7.5 7.5 0 1115 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077l1.41-.513m14.095-5.13l1.41-.513M5.106 17.785l1.15-.964m11.49-9.642l1.149-.964M7.501 19.795l.75-1.3m7.5-12.99l.75-1.3m-6.063 16.658l.26-1.477m2.605-14.772l.26-1.477m0 17.726l-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205L12 12m6.894 5.785l-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864l-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Powerful API</h3>
                <p class='mt-5 text-base text-gray-500'>Ab a facere voluptatem in quia corrupti veritatis aliquam.
                  Veritatis labore quaerat ipsum quaerat id.</p>
              </div>
            </div>
          </div>

          <div class='pt-6'>
            <div class='flow-root rounded-lg bg-gray-50 px-6 pb-8'>
              <div class='-mt-6'>
                <div>
                  <span
                    class='inline-flex items-center justify-center rounded-md bg-gradient-to-r from-teal-500 to-cyan-600 p-3 shadow-lg'>
                    <!-- Heroicon name: outline/server -->
                    <svg class='h-6 w-6 text-white' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'
                      stroke-width='1.5' stroke='currentColor' aria-hidden='true'>
                      <path stroke-linecap='round' stroke-linejoin='round'
                        d='M21.75 17.25v-.228a4.5 4.5 0 00-.12-1.03l-2.268-9.64a3.375 3.375 0 00-3.285-2.602H7.923a3.375 3.375 0 00-3.285 2.602l-2.268 9.64a4.5 4.5 0 00-.12 1.03v.228m19.5 0a3 3 0 01-3 3H5.25a3 3 0 01-3-3m19.5 0a3 3 0 00-3-3H5.25a3 3 0 00-3 3m16.5 0h.008v.008h-.008v-.008zm-3 0h.008v.008h-.008v-.008z' />
                    </svg>
                  </span>
                </div>
                <h3 class='mt-8 text-lg font-medium tracking-tight text-gray-900'>Database Backups</h3>
                <p class='mt-5 text-base text-gray-500'>Quia qui et est officia cupiditate qui consectetur. Ratione
                  similique et impedit ea ipsum et.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Testimonial section -->
  <div class='bg-gradient-to-r from-teal-500 to-cyan-600 pb-16 lg:relative lg:z-10 lg:pb-0'>
    <div class='lg:mx-auto lg:grid lg:max-w-7xl lg:grid-cols-3 lg:gap-8 lg:px-8'>
      <div class='relative lg:-my-8'>
        <div aria-hidden='true' class='absolute inset-x-0 top-0 h-1/2 bg-white lg:hidden'></div>
        <div class='mx-auto max-w-md px-6 sm:max-w-3xl lg:h-full lg:p-0'>
          <div
            class='aspect-w-10 aspect-h-6 overflow-hidden rounded-xl shadow-xl sm:aspect-w-16 sm:aspect-h-7 lg:aspect-none lg:h-full'>
            <img class='object-cover lg:h-full lg:w-full'
              src='https://images.unsplash.com/photo-1520333789090-1afc82db536a?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=2102&q=80'
              alt=''>
          </div>
        </div>
      </div>
      <div class='mt-12 lg:col-span-2 lg:m-0 lg:pl-8'>
        <div class='mx-auto max-w-md px-6 sm:max-w-2xl lg:max-w-none lg:px-0 lg:py-20'>
          <blockquote>
            <div>
              <svg class='h-12 w-12 text-white opacity-25' fill='currentColor' viewBox='0 0 32 32' aria-hidden='true'>
                <path
                  d='M9.352 4C4.456 7.456 1 13.12 1 19.36c0 5.088 3.072 8.064 6.624 8.064 3.36 0 5.856-2.688 5.856-5.856 0-3.168-2.208-5.472-5.088-5.472-.576 0-1.344.096-1.536.192.48-3.264 3.552-7.104 6.624-9.024L9.352 4zm16.512 0c-4.8 3.456-8.256 9.12-8.256 15.36 0 5.088 3.072 8.064 6.624 8.064 3.264 0 5.856-2.688 5.856-5.856 0-3.168-2.304-5.472-5.184-5.472-.576 0-1.248.096-1.44.192.48-3.264 3.456-7.104 6.528-9.024L25.864 4z' />
              </svg>
              <p class='mt-6 text-2xl font-medium text-white'>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed
                urna nulla vitae laoreet augue. Amet feugiat est integer dolor auctor adipiscing nunc urna, sit.</p>
            </div>
            <footer class='mt-6'>
              <p class='text-base font-medium text-white'>Judith Black</p>
              <p class='text-base font-medium text-cyan-100'>CEO at PureInsights</p>
            </footer>
          </blockquote>
        </div>
      </div>
    </div>
  </div>

  <!-- Blog section -->
  <div class='relative bg-gray-50 py-16 sm:py-24 lg:py-32'>
    <div class='relative'>
      <div class='mx-auto max-w-md px-6 text-center sm:max-w-3xl lg:max-w-7xl lg:px-8'>
        <h2 class='text-lg font-semibold text-cyan-600'>Learn</h2>
        <p class='mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl'>Helpful Resources</p>
        <p class='mx-auto mt-5 max-w-prose text-xl text-gray-500'>Phasellus lorem quam molestie id quisque diam aenean
          nulla in. Accumsan in quis quis nunc, ullamcorper malesuada. Eleifend condimentum id viverra nulla.</p>
      </div>
      <div class='mx-auto mt-12 grid max-w-md gap-8 px-6 sm:max-w-lg lg:max-w-7xl lg:grid-cols-3 lg:px-8'>
        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1496128858413-b36217c2ce36?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Article</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>Boost your conversion rate</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit.
                  Architecto accusantium praesentium eius, ut atque fuga culpa, similique sequi cum eos quis dolorum.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Roel Aufderehar'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Roel Aufderehar</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-03-16'>Mar 16, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>6 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1547586696-ea22b4d4235d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Video</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>How to use search engine optimization to drive sales</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit. Velit
                  facilis asperiores porro quaerat doloribus, eveniet dolore. Adipisci tempora aut inventore optio animi.,
                  tempore temporibus quo laudantium.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Brenna Goyette'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Brenna Goyette</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-03-10'>Mar 10, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>4 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class='flex flex-col overflow-hidden rounded-lg shadow-lg'>
          <div class='flex-shrink-0'>
            <img class='h-48 w-full object-cover'
              src='https://images.unsplash.com/photo-1492724441997-5dc865305da7?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1679&q=80'
              alt=''>
          </div>
          <div class='flex flex-1 flex-col justify-between bg-white p-6'>
            <div class='flex-1'>
              <p class='text-sm font-medium text-cyan-600'>
                <a href='#' class='hover:underline'>Case Study</a>
              </p>
              <a href='#' class='mt-2 block'>
                <p class='text-xl font-semibold text-gray-900'>Improve your customer experience</p>
                <p class='mt-3 text-base text-gray-500'>Lorem ipsum dolor sit amet consectetur adipisicing elit. Sint
                  harum rerum voluptatem quo recusandae magni placeat saepe molestiae, sed excepturi cumque corporis
                  perferendis hic.</p>
              </a>
            </div>
            <div class='mt-6 flex items-center'>
              <div class='flex-shrink-0'>
                <a href='#'>
                  <img class='h-10 w-10 rounded-full'
                    src='https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80'
                    alt='Daniela Metz'>
                </a>
              </div>
              <div class='ml-3'>
                <p class='text-sm font-medium text-gray-900'>
                  <a href='#' class='hover:underline'>Daniela Metz</a>
                </p>
                <div class='flex space-x-1 text-sm text-gray-500'>
                  <time datetime='2020-02-12'>Feb 12, 2020</time>
                  <span aria-hidden='true'>&middot;</span>
                  <span>11 min read</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- CTA Section -->
  <div class='relative bg-gray-900'>
    <div class='relative h-56 bg-indigo-600 sm:h-72 md:absolute md:left-0 md:h-full md:w-1/2'>
      <img class='h-full w-full object-cover'
        src='https://images.unsplash.com/photo-1525130413817-d45c1d127c42?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1920&q=60&sat=-100'
        alt=''>
      <div aria-hidden='true' class='absolute inset-0 bg-gradient-to-r from-teal-500 to-cyan-600 mix-blend-multiply'>
      </div>
    </div>
    <div class='relative mx-auto max-w-md py-12 px-6 sm:max-w-7xl sm:py-20 md:py-28 lg:px-8 lg:py-32'>
      <div class='md:ml-auto md:w-1/2 md:pl-10'>
        <h2 class='text-lg font-semibold text-gray-300'>Award winning support</h2>
        <p class='mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl'>We\xE2\x80\x99re here to help</p>
        <p class='mt-3 text-lg text-gray-300'>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Et, egestas tempus
          tellus etiam sed. Quam a scelerisque amet ullamcorper eu enim et fermentum, augue. Aliquet amet volutpat quisque
          ut interdum tincidunt duis.</p>
        <div class='mt-8'>
          <div class='inline-flex rounded-md shadow'>
            <a href='#'
              class='inline-flex items-center justify-center rounded-md border border-transparent bg-white px-5 py-3 text-base font-medium text-gray-900 hover:bg-gray-50'>
              Visit the help center
              <!-- Heroicon name: mini/arrow-top-right-on-square -->
              <svg class='-mr-1 ml-3 h-5 w-5 text-gray-400' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'
                fill='currentColor' aria-hidden='true'>
                <path fill-rule='evenodd'
                  d='M4.25 5.5a.75.75 0 00-.75.75v8.5c0 .414.336.75.75.75h8.5a.75.75 0 00.75-.75v-4a.75.75 0 011.5 0v4A2.25 2.25 0 0112.75 17h-8.5A2.25 2.25 0 012 14.75v-8.5A2.25 2.25 0 014.25 4h5a.75.75 0 010 1.5h-5z'
                  clip-rule='evenodd' />
                <path fill-rule='evenodd'
                  d='M6.194 12.753a.75.75 0 001.06.053L16.5 4.44v2.81a.75.75 0 001.5 0v-4.5a.75.75 0 00-.75-.75h-4.5a.75.75 0 000 1.5h2.553l-9.056 8.194a.75.75 0 00-.053 1.06z'
                  clip-rule='evenodd' />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</main>
{% endblock content %}",)?;
    Ok(())
}
