$(document).ready(function() {
    console.log('o');
    $(document).on('click', '.complete-button', function() {
        var task = JSON.stringify({
            id: $(this).parent().data('id'),
            name: $(this).parent().find('.name').text(),
            complete: !$(this).data('complete')
        });
        $.ajax({
            url: '/',
            type: 'PUT',
            contentType: 'application/json; charset=UTF-8',
            data: task,
            success: function(response) {
                console.log('success');
                window.location.replace("/");
            },
            error: function(err) {
                console.log('err', err);
            }
        });
    });
    $(document).on('click', '.delete-button', function() {
        var task = JSON.stringify({
            id: $(this).parent().data('id'),
            name: '',
            complete: false
        });
        $.ajax({
            url: '/',
            type: 'DELETE',
            contentType: 'application/json',
            data: task,
            success: function(response) {
                console.log('success');
                window.location.replace('/');
            },
            error: function(err) {
                console.log('err');
            }
        });
    });
});
