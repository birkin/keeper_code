#[macro_use] extern crate rocket;


// see <https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/hello/src/main.rs>



// in this section, get a route to '/info', and a route to '/version' to work

#[get("/")]
async fn index() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: root-response"
}



// #[get("/version")]
// async fn version() -> &'static str {
//     use rocket::tokio::time::{sleep, Duration};
//     sleep(Duration::from_secs(2)).await;
//     "coming: version-response"
// }

#[get("/version")]
async fn version() -> &'static str {

    // use rocket::tokio::time::{sleep, Duration}; // original line 1
    use rocket::tokio::time::{sleep, Duration, Instant};

    let start = Instant::now();
    // let zz: () = start;  // yields: found struct `std::time::Instant`

    sleep(Duration::from_secs(1)).await;        // original line 2

    let elapsed: Duration = start.elapsed();
    // let zz: () = elapsed;  // yields: found struct `Duration`

    println!( "elapsed time, `{:?}`", elapsed );

    "coming: version-response"                  // original line 3
}




#[get("/info")]
async fn info() -> &'static str {
    use rocket::tokio::time::{sleep, Duration};
    sleep(Duration::from_secs(2)).await;
    "coming: info-response"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![version])
        .mount("/", routes![info])
}



// python version code -- start

// def version( request ):
//     """ Returns basic branch and commit data. """
//     rq_now = datetime.datetime.now()
//     commit = version_helper.get_commit()
//     branch = version_helper.get_branch()
//     info_txt = commit.replace( 'commit', branch )
//     context = version_helper.make_context( request, rq_now, info_txt )
//     output = json.dumps( context, sort_keys=True, indent=2 )
//     return HttpResponse( output, content_type='application/json; charset=utf-8' )

// def get_commit():
//     """ Returns commit-string.
//         Called by views.version() """
//     original_directory = os.getcwd()
//     log.debug( 'BASE_DIR, ```%s```' % settings.BASE_DIR )
//     git_dir = settings.BASE_DIR
//     os.chdir( git_dir )
//     output8 = subprocess.check_output( ['git', 'log'], stderr=subprocess.STDOUT )
//     output = output8.decode( 'utf-8' )
//     os.chdir( original_directory )
//     lines = output.split( '\n' )
//     commit = lines[0]
//     return commit


// def get_branch():
//     """ Returns branch.
//         Called by views.version() """
//     original_directory = os.getcwd()
//     git_dir = settings.BASE_DIR
//     os.chdir( git_dir )
//     output8 = subprocess.check_output( ['git', 'branch'], stderr=subprocess.STDOUT )
//     output = output8.decode( 'utf-8' )
//     os.chdir( original_directory )
//     lines = output.split( '\n' )
//     branch = 'init'
//     for line in lines:
//         if line[0:1] == '*':
//             branch = line[2:]
//             break
//     return branch


// def make_context( request, rq_now, info_txt ):
//     """ Assembles data-dct.
//         Called by views.version() """
//     context = {
//         'request': {
//         'url': '%s://%s%s' % (
//             request.scheme,
//             request.META.get( 'HTTP_HOST', '127.0.0.1' ),  # HTTP_HOST doesn't exist for client-tests
//             request.META.get('REQUEST_URI', request.META['PATH_INFO'])
//             ),
//         'timestamp': str( rq_now )
//         },
//         'response': {
//             'version': info_txt,
//             'timetaken': str( datetime.datetime.now() - rq_now )
//         }
//     }
//     return context

// python version code -- end

