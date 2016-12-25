$(document).ready(function() {
    console.log('o');
    $(document).on('click', '.complete', function() {
        // var complete = false;
        // if($(this).text() === 'Incomplete') {
        //     complete = true;
        // }
        var task = JSON.stringify({
            id: $(this).parent().data('id'),
            name: $(this).parent().find('.name').text(),
            complete: !$(this).data('complete')
        });
        $.ajax({
            url: '/',
            type: 'PUT',
            dataType: 'json',
            contentType: 'application/json; charset=UTF-8',
            data: task,
            success: function(response) {
                console.log('success');
            },
            error: function(err) {
                console.log('err', err);
                window.location.replace("/");
            }
        });
    });
});
