use crate::writers::write_to_file;
use crate::generators::create_file;
use color_eyre::eyre::Result;
use eyre::Error;


pub fn write_to_edit_page_html() -> Result<(), Error> {
    let contents: String = r#"
{% extends 'layouts/authenticated_page/authenticated_page.html.tera' %}
{% block title %}{{title | default(value="Dashboard", boolean=true)}}{% endblock title %}

{% block authenticated_content %}
{{ super() }}



<div style="height: 92vh; width: 100%;">
    {% include 'components/grapesjs.html.tera' ignore missing %}
</div>


<style>
    body,
    html {
        height: 100%;
        margin: 0;
    }

    .gjs-block {
        padding: 0 !important;
        width: 100% !important;
        min-height: auto !important;
    }

    .gjs-block svg {
        width: 100%;
    }

    .change-theme-button {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        margin: 5px;
    }

    .change-theme-button:focus {
        /* background-color: yellow; */
        outline: none;
        box-shadow: 0 0 0 2pt #c5c5c575;
    }


    .gjs-pn-views-container {
        height: auto !important;
    }
</style>

<script>
    const escapeName = (name) => `${name}`.trim().replace(/([^a-z0-9\w-:/]+)/gi, '-');

    window.editor = grapesjs.init({
        height: '100%',
        width: '100%',
        container: '#gjs',
        showOffsets: true,
        allowScripts: 1,
        fromElement: true,
        noticeOnUnload: false,
        storageManager: false,
        selectorManager: { escapeName },
        plugins: [
            'grapesjs-tailwind',
            'grapesjs-preset-webpage',
            'grapesjs-script-editor',
        ],
        pluginsOpts: {
            'grapesjs-tailwind': { /* Test here your options  */ },
            'granpesjs-preset-webpage': { /* Test here your options  */ },
            'grapesjs-script-editor': { /* Test here your options  */ },
        }
    });


    editor.Panels.addButton('options', {
        id: 'update-theme',
        className: 'fa fa-adjust',
        command: 'open-update-theme',
        attributes: {
            title: 'Update Theme',
            'data-tooltip-pos': 'bottom',
        },
    });

    let isSaved = false;

    const saveHtml = (HtmlGrapesJs, sender) => {
        if (!isSaved) {
            // save html to database
            fetch('/page/{{page.id}}', {
                method: 'PATCH', // or 'PUT'
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(
                    HtmlGrapesJs
                ),
            })
                .then(response => response.json())
                .then(data => {
                    console.log('Success:', data);
                    sender.set('active', 1); // turn on the button
                })
                .catch((error) => {
                    console.error('Error:', error);
                    sender.set('active', 1); // turn on the button
                });
            isSaved = true;
        }
    };

    editor.Commands.add('savePage', {
        run(editor, sender) {
            sender.set('active', 0); // turn off the button
            // get html from editor
            var html = editor.getHtml();
            const HtmlGrapesJs = {
                html_content: html,
                associated_user_id: 1,
                summary: '{% if page %}{{page.summary}}{% else %}{% endif %}',
                title: '{% if page %}{{page.title}}{% else %}{% endif %}',
                created_at: '{% if page %}{{page.created_at}}{% else %}{% endif %}',
                author: '{% if page %}{{page.author}}{% else %}{% endif %}',
                excerpt: `{% if page %}{{page.excerpt}}{% else %}{% endif %}`,
                slug: '{% if page %}{{page.slug}}{% else %}{% endif %}',
             };

            saveHtml(HtmlGrapesJs, sender);
        }
    });

    editor.Panels.addButton('options', {
        id: 'savePage',
        className: 'fa fa-save',
        command: 'savePage',
        attributes: {
            title: 'Save HTML',
            'data-tooltip-pos': 'bottom',

        },
    });

    // declare the db
    const dbPromise = idb.openDB('page', 1, {
      upgrade(db) {
          db.createObjectStore('keyval');
      },
    });

    // Function to save page to indexedDB
    const savePageToIndexedDB = async (page) => {

        // check if the new title is different from the old title
        if (page.title !== title_value) {
            // if it is different then delete the old page from indexedDB
            deletePageFromIndexedDB(title_value);
            console.log('Deleted old page from indexedDB');
        }

        const db = await dbPromise;
        const tx = db.transaction('page', 'readwrite');
        const store = tx.objectStore('page');
        // Remove the key, as the key is in-line and will be automatically generated from the object
        await store.put(page);
        return tx.complete;
    };


    // Function to get page from indexedDB
    const getPageFromIndexedDB = async (key) => {
        const db = await dbPromise;
        const tx = db.transaction('page', 'readonly');
        const store = tx.objectStore('page');
        return store.get(key);
    };

    // Function to delete page from indexedDB
    const deletePageFromIndexedDB = async (key) => {
        const db = await dbPromise;
        const tx = db.transaction('page', 'readwrite');
        const store = tx.objectStore('page');
        await store.delete(key);
        return tx.complete;
    };

    async function BuildPageContent() {
        try {
            const page = await getPageFromIndexedDB(0);
            if (page) {

                const title = page.title
                const html_content = editor.getHtml();
                const associated_user_id = page.associated_user_id ? page.associated_user_id : 1;
                const author = page.author;
                const summary = page.summary;
                const excerpt = page.excerpt;
                const slug = page.slug;
                const pageStatus = page.pageStatus ? page.pageStatus : 'draft';
                const author_image = page.author_image ? page.author_image : '';
                const authorImageThumbnail = page.authorImageThumbnail ? page.authorImageThumbnail : '';
                const authorUrl = page.authorUrl ? page.authorUrl : '';
                const featured_image = page.featured_image ? page.featured_image : '';
                const featuredImageThumbnail = page.featuredImageThumbnail ? page.featuredImageThumbnail : '';
                const seoTitle = page.seoTitle ? page.seoTitle : '';
                const seoDescription = page.seoDescription ? page.seoDescription : '';
                const seoKeywords = page.seoKeywords ? page.seoKeywords : '';
                const seoFocusKeyword = page.seoFocusKeyword ? page.seoFocusKeyword : '';
                const seoCanonical = page.seoCanonical ? page.seoCanonical : '';
                const seoNoIndex = page.seoNoIndex ? page.seoNoIndex : '';
                const seoNoFollow = page.seoNoFollow ? page.seoNoFollow : '';
                const seoOGTitle = page.seoOGTitle ? page.seoOGTitle : '';
                const seoOGLocale = page.seoOGLocale ? page.seoOGLocale : '';
                const seoOGType = page.seoOGType ? page.seoOGType : '';
                const seoOGDescription = page.seoOGDescription ? page.seoOGDescription : '';
                const seoOGImage = page.seoOGImage ? page.seoOGImage : '';
                const ogImageWidth = page.ogImageWidth ? page.ogImageWidth : '';
                const ogImageHeight = page.ogImageHeight ? page.ogImageHeight : '';
                const seoTwitterTitle = page.seoTwitterTitle ? page.seoTwitterTitle : '';
                const seoTwitterDescription = page.seoTwitterDescription ? page.seoTwitterDescription : '';
                const seoTwitterImage = page.seoTwitterImage ? page.seoTwitterImage : '';
                const twitterImageAlt = page.twitterImageAlt ? page.twitterImageAlt : '';
                const seoTwitterCard = page.seoTwitterCard ? page.seoTwitterCard : '';
                const schemaType = page.schemaType ? page.schemaType : '';
                const schemaPageType = page.schemaPageType ? page.schemaPageType : '';
                const schemaArticleType = page.schemaArticleType ? page.schemaArticleType : '';
                const schemaDescription = page.schemaDescription ? page.schemaDescription : '';
                const schemaAuthor = page.schemaAuthor ? page.schemaAuthor : '';
                const schemaPublisher = page.schemaPublisher ? page.schemaPublisher : '';
                const schemaImage = page.schemaImage ? page.schemaImage : '';
                const schemaUrl = page.schemaUrl ? page.schemaUrl : '';
                const schemaName = page.schemaName ? page.schemaName : '';
                const schemaHeadline = page.schemaHeadline ? page.schemaHeadline : '';
                const schemaDatePublished = page.schemaDatePublished ? page.schemaDatePublished : '';
                const schemaDateModified = page.schemaDateModified ? page.schemaDateModified : '';


                const now = Date.now();
                // get the values from the form


                const page_content = {
                    title,
                    html_content,
                    associated_user_id,
                    author,
                    summary,
                    excerpt,
                    slug,
                    pageStatus,
                    author_image,
                    authorImageThumbnail,
                    authorUrl,
                    featured_image,
                    featuredImageThumbnail,
                    seoTitle,
                    seoDescription,
                    seoKeywords,
                    seoFocusKeyword,
                    seoCanonical,
                    seoNoIndex,
                    seoNoFollow,
                    seoOGTitle,
                    seoOGLocale,
                    seoOGType,
                    seoOGDescription,
                    seoOGImage,
                    ogImageWidth,
                    ogImageHeight,
                    seoTwitterTitle,
                    seoTwitterDescription,
                    seoTwitterImage,
                    twitterImageAlt,
                    seoTwitterCard,
                    schemaType,
                    schemaPageType,
                    schemaArticleType,
                    schemaDescription,
                    schemaAuthor,
                    schemaPublisher,
                    schemaImage,
                    schemaUrl,
                    schemaName,
                    schemaHeadline,
                    schemaDatePublished,
                    schemaDateModified
                };

                return page_content;

            } else {
                console.log('No page found in indexedDB');
            }

        } catch (error) {
            console.log(error);
        }
    }


</script>
{% endblock authenticated_content %}
    "#
        .to_string();

    create_file("src/views/layouts/authenticated_page/page/edit_page.html.tera")
        .unwrap_or_else(|_| panic!("Error: Could not create edit_page.html.tera"));

    write_to_file(
        "src/views/layouts/authenticated_page/page/edit_page.html.tera",
        contents.as_bytes(),
    )
        .unwrap_or_else(|_| panic!("Error: Could not write to edit_page.html"));
    Ok(())
}
