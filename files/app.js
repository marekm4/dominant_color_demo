var changeStatus = function (status) {
    $('#status').text(status);
};

var changeStatusToAnalyzing = function (image) {
    changeStatus('Analyzing...');
    loadImage(
        image,
        updateImagePreview,
        {
            maxWidth: 480,
            maxHeight: 320,
            contain: true,
        }
    );
};

var updateImagePreview = function(img) {
    var preview = document.getElementById('preview-container');
    preview.appendChild(img);
    preview.removeChild(preview.firstChild);
}

var changeStatusToError = function () {
    changeStatus('Error');
};

var changeStatusToResult = function (result) {
    var status = document.getElementById('status');
    status.innerHTML = "";
    var colors = result.colors;
    var hasAlpha = result.has_alpha;
    var size = hasAlpha ? 4 : 3;
    for (var i = 0; i < colors.length; i += size) {
        $(status).append('<div class="color" style="background-color:rgb(' + colors[i] + ', ' + colors[i + 1] + ', ' + colors[i + 2] + ')"></div>');
    }
};

$('.example-image').click(function (event) {
    var image = $(event.target);
    var url = image.attr('src');
    changeStatusToAnalyzing(url);
    $.post('url', {url: url}).done(function (data) {
        changeStatusToResult(data);
    }).fail(function () {
        changeStatusToError();
    });
});

$("#url-form input").bind('input', function (event) {
    var form = $(event.target).parent();
    var url = form.find('input').val();
    changeStatusToAnalyzing(url);
    $.post('url.php', {url: url}).done(function (data) {
        changeStatusToResult(data);
    }).fail(function () {
        changeStatusToError();
    });
    event.preventDefault();
});

$("#upload-form input").change(function (event) {
    event.preventDefault();
    var form = $(event.target).parent();
    var fileInput = form.find('input');

    var formData = new FormData();
    formData.append(fileInput.attr('name'), fileInput[0].files[0]);
    var request = new XMLHttpRequest();

    request.onreadystatechange = function () {
        if (request.readyState == 4) {
            if (request.status == 200) {
                changeStatusToResult(JSON.parse(request.responseText));
            } else {
                changeStatusToError();
            }
        }
    };

    changeStatusToAnalyzing(event.target.files[0]);
    request.open("POST", form.attr('action'));
    request.send(formData);
});
