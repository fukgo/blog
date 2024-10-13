import ProtectedComponent from '../../components/ProtectedComponent';
import TagsList from '../../components/TagsList';

const NewTag = () => {
    return (
        <div>
            <TagsList />
        </div>
    );
};

export default function NewTagPage() {
    return (
        <ProtectedComponent>
            <NewTag />
        </ProtectedComponent>
    );
}